use std::ffi::c_void;
use std::ptr::NonNull;
use std::sync::{Arc, Mutex};

use ash::vk;
use gpu_allocator::MemoryLocation;
use gpu_allocator::vulkan::{AllocationCreateDesc, AllocatorCreateDesc};
use crate::prelude::DeviceFunctions;

#[derive(Debug)]
pub enum AllocationError {
    GpuAllocator(gpu_allocator::AllocationError),
}

impl From<gpu_allocator::AllocationError> for AllocationError {
    fn from(err: gpu_allocator::AllocationError) -> Self {
        Self::GpuAllocator(err)
    }
}

pub enum AllocationStrategy {
    /// Automatically select memory that is only used by the gpu
    AutoGpuOnly,

    /// Automatically select memory that is used by both gpu and cpu
    AutoGpuCpu,
}

/// Manages memory allocation for vulkan object
///
/// Currently just uses the [`gpu_allocator::vulkan::Allocator`] struct.
pub struct Allocator {
    device: Arc<DeviceFunctions>,
    allocator: Mutex<gpu_allocator::vulkan::Allocator>
}

impl Allocator {
    pub fn new(device: Arc<DeviceFunctions>) -> Self {
        let allocator = gpu_allocator::vulkan::Allocator::new(&AllocatorCreateDesc{
            instance: device.instance.vk().clone(),
            device: device.vk.clone(),
            physical_device: device.physical_device,
            debug_settings: Default::default(),
            buffer_device_address: false
        }).unwrap();

        Self {
            device,
            allocator: Mutex::new(allocator),
        }
    }

    pub fn allocate_buffer_memory(&self, buffer: vk::Buffer, strategy: &AllocationStrategy) -> Result<Allocation, AllocationError> {
        let location = match strategy {
            AllocationStrategy::AutoGpuOnly => MemoryLocation::GpuOnly,
            AllocationStrategy::AutoGpuCpu => MemoryLocation::CpuToGpu,
        };

        let requirements = unsafe {
            self.device.vk.get_buffer_memory_requirements(buffer)
        };

        let alloc_desc = AllocationCreateDesc{
            name: "",
            requirements,
            location,
            linear: true
        };

        let alloc = self.allocator.lock().unwrap().allocate(&alloc_desc)?;

        Ok(Allocation::new(alloc))
    }

    pub fn allocate_image_memory(&self, image: vk::Image, strategy: &AllocationStrategy) -> Result<Allocation, AllocationError> {
        let location = match strategy {
            AllocationStrategy::AutoGpuOnly => MemoryLocation::GpuOnly,
            AllocationStrategy::AutoGpuCpu => MemoryLocation::CpuToGpu,
        };

        let requirements = unsafe {
            self.device.vk.get_image_memory_requirements(image)
        };

        let alloc_desc = AllocationCreateDesc{
            name: "",
            requirements,
            location,
            // If image is accessed by the cpu it has to be linear
            linear: location == MemoryLocation::CpuToGpu,
        };

        let alloc = self.allocator.lock().unwrap().allocate(&alloc_desc)?;

        Ok(Allocation::new(alloc))
    }

    pub fn free(&self, allocation: Allocation) {
        self.allocator.lock().unwrap().free(allocation.alloc).unwrap()
    }
}

#[derive(Debug)]
pub struct Allocation {
    alloc: gpu_allocator::vulkan::Allocation,
}

impl Allocation {
    fn new(alloc: gpu_allocator::vulkan::Allocation) -> Self {
        Self {
            alloc,
        }
    }

    pub fn mapped_ptr(&self) -> Option<std::ptr::NonNull<c_void>> {
        self.alloc.mapped_ptr()
    }

    pub fn memory(&self) -> vk::DeviceMemory {
        unsafe { self.alloc.memory() }
    }

    pub fn offset(&self) -> vk::DeviceSize {
        self.alloc.offset()
    }
}

pub struct MappedMemory {
    ptr: NonNull<c_void>,
    size: usize,
}

impl MappedMemory {
    pub unsafe fn new(ptr: NonNull<c_void>, size: usize) -> Self {
        if size == 0 {
            panic!("Size of mapped memory must be greater than 0.");
        }
        Self {
            ptr,
            size,
        }
    }

    pub fn as_byte_slice_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr() as *mut u8, self.size) }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }
}