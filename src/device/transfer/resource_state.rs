use std::collections::HashMap;

use ash::vk;

use crate::vk::objects::buffer::{Buffer, BufferId};
use crate::vk::objects::image::{Image, ImageId};

use crate::prelude::*;

pub struct BufferStateTracker {
    buffers: HashMap<BufferId, BufferState>,
}

impl BufferStateTracker {
    pub fn new() -> Self {
        Self {
            buffers: HashMap::new(),
        }
    }

    /// Registers a buffer into the tracker.
    ///
    /// The buffer is initialized to having no pending reads or writes.
    ///
    /// If the buffer is already registered [`Err`] is returned.
    pub fn register(&mut self, buffer: Buffer) -> Result<(), ()> {
        if self.buffers.contains_key(&buffer.get_id()) {
            return Err(());
        }
        self.buffers.insert(buffer.get_id(), BufferState::new(buffer));
        Ok(())
    }

    /// Updates the state of a buffer, records any required barriers and returns the handle of the
    /// buffer.
    ///
    /// If the buffer could not be found [`None`] is returned.
    pub fn update_state(&mut self, id: BufferId, read: bool, write: bool, barriers: &mut Vec<vk::BufferMemoryBarrier2>) -> Option<vk::Buffer> {
        if let Some(buffer) = self.buffers.get_mut(&id) {
            buffer.update_state(read, write, barriers);
            Some(buffer.handle)
        } else {
            None
        }
    }

    /// Releases a registered buffer returning its handle and [`ash::vk::AccessFlags2`] representing
    /// any pending operations on the buffer.
    ///
    /// If the buffer could not be found [`None`] is returned.
    pub fn release(&mut self, id: BufferId) -> Option<(vk::Buffer, vk::AccessFlags2)> {
        if let Some(buffer) = self.buffers.remove(&id) {
            let mut access_mask = vk::AccessFlags2::empty();
            if buffer.read_pending {
                access_mask |= vk::AccessFlags2::TRANSFER_READ;
            }
            if buffer.write_pending {
                access_mask |= vk::AccessFlags2::TRANSFER_WRITE;
            }

            Some((buffer.handle, access_mask))
        } else {
            None
        }
    }
}

struct BufferState {
    handle: vk::Buffer,
    read_pending: bool,
    write_pending: bool,
}

impl BufferState {
    fn new(buffer: Buffer) -> Self {
        Self {
            handle: buffer.get_handle(),
            read_pending: false,
            write_pending: false,
        }
    }

    fn update_state(&mut self, read: bool, write: bool, barriers: &mut Vec<vk::BufferMemoryBarrier2>) {
        let mut src_access_mask = vk::AccessFlags2::empty();
        if read && self.write_pending {
            src_access_mask |= vk::AccessFlags2::TRANSFER_WRITE;
        }
        if write && (self.write_pending || self.read_pending) {
            src_access_mask |= vk::AccessFlags2::TRANSFER_WRITE | vk::AccessFlags2::TRANSFER_READ;
        }
        self.read_pending |= read;
        self.write_pending |= write;

        if src_access_mask != vk::AccessFlags2::empty() {
            barriers.push(vk::BufferMemoryBarrier2::builder()
                .src_stage_mask(vk::PipelineStageFlags2::TRANSFER)
                .src_access_mask(src_access_mask)
                .dst_stage_mask(vk::PipelineStageFlags2::TRANSFER)
                .dst_access_mask(vk::AccessFlags2::TRANSFER_READ | vk::AccessFlags2::TRANSFER_WRITE)
                .buffer(self.handle)
                .offset(0)
                .size(vk::WHOLE_SIZE)
                .build()
            );
        }
    }
}

pub struct ImageStateTracker {
    images: HashMap<ImageId, ImageState>,
}

impl ImageStateTracker {
    pub fn new() -> Self {
        Self {
            images: HashMap::new(),
        }
    }

    pub fn register(&mut self, image: Image, aspect_mask: vk::ImageAspectFlags, layout: vk::ImageLayout) -> Result<(), ()> {
        if self.images.contains_key(&image.get_id()) {
            return Err(());
        }
        self.images.insert(image.get_id(), ImageState::new(image, aspect_mask, layout));
        Ok(())
    }

    pub fn update_state_read(&mut self, image: ImageId, barriers: &mut Vec<vk::ImageMemoryBarrier2>) -> Option<vk::Image> {
        if let Some(image) = self.images.get_mut(&image) {
            image.update_state_read(barriers);
            Some(image.handle)
        } else {
            None
        }
    }

    pub fn update_state_write(&mut self, image: ImageId, barriers: &mut Vec<vk::ImageMemoryBarrier2>) -> Option<vk::Image> {
        if let Some(image) = self.images.get_mut(&image) {
            image.update_state_write(barriers);
            Some(image.handle)
        } else {
            None
        }
    }

    pub fn release(&mut self, id: ImageId) -> Option<(vk::Image, vk::ImageAspectFlags, vk::AccessFlags2, vk::ImageLayout)> {
        if let Some(image) = self.images.remove(&id) {
            let mut access_mask = vk::AccessFlags2::empty();
            if image.read_pending {
                access_mask |= vk::AccessFlags2::TRANSFER_READ;
            }
            if image.write_pending {
                access_mask |= vk::AccessFlags2::TRANSFER_WRITE;
            }

            Some((image.handle, image.aspect_mask, access_mask, image.layout))
        } else {
            None
        }
    }
}

struct ImageState {
    handle: vk::Image,
    aspect_mask: vk::ImageAspectFlags,
    layout: vk::ImageLayout,
    read_pending: bool,
    write_pending: bool,
}

impl ImageState {
    fn new(image: Image, aspect_mask: vk::ImageAspectFlags, layout: vk::ImageLayout) -> Self {
        Self {
            handle: image.get_handle(),
            aspect_mask,
            layout,
            read_pending: false,
            write_pending: false,
        }
    }

    fn update_state_read(&mut self, barriers: &mut Vec<vk::ImageMemoryBarrier2>) {
        if self.layout != vk::ImageLayout::TRANSFER_SRC_OPTIMAL || self.write_pending {
            barriers.push(vk::ImageMemoryBarrier2::builder()
                .src_stage_mask(vk::PipelineStageFlags2::TRANSFER)
                .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
                .dst_stage_mask(vk::PipelineStageFlags2::TRANSFER)
                .dst_access_mask(vk::AccessFlags2::TRANSFER_READ)
                .old_layout(self.layout)
                .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
                .image(self.handle)
                .subresource_range(vk::ImageSubresourceRange {
                    aspect_mask: self.aspect_mask,
                    base_mip_level: 0,
                    level_count: vk::REMAINING_MIP_LEVELS,
                    base_array_layer: 0,
                    layer_count: vk::REMAINING_ARRAY_LAYERS
                })
                .build()
            );

            self.layout = vk::ImageLayout::TRANSFER_SRC_OPTIMAL;
            self.write_pending = false;
            self.read_pending = true;
        }
    }

    fn update_state_write(&mut self, barriers: &mut Vec<vk::ImageMemoryBarrier2>) {
        if self.layout != vk::ImageLayout::TRANSFER_DST_OPTIMAL || self.read_pending || self.write_pending {
            barriers.push(vk::ImageMemoryBarrier2::builder()
                .src_stage_mask(vk::PipelineStageFlags2::TRANSFER)
                .src_access_mask(vk::AccessFlags2::TRANSFER_READ | vk::AccessFlags2::TRANSFER_WRITE)
                .dst_stage_mask(vk::PipelineStageFlags2::TRANSFER)
                .dst_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
                .old_layout(self.layout)
                .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                .image(self.handle)
                .subresource_range(vk::ImageSubresourceRange {
                    aspect_mask: self.aspect_mask,
                    base_mip_level: 0,
                    level_count: vk::REMAINING_MIP_LEVELS,
                    base_array_layer: 0,
                    layer_count: vk::REMAINING_ARRAY_LAYERS
                })
                .build()
            );

            self.layout = vk::ImageLayout::TRANSFER_DST_OPTIMAL;
            self.write_pending = true;
            self.read_pending = false;
        }
    }
}