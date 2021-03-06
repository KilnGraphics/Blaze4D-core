use std::collections::HashSet;
use std::sync::Arc;

use ash::vk;

use crate::renderer::emulator::immediate::ImmediateBuffer;
use crate::renderer::emulator::{GlobalImage, GlobalMesh, MeshData};
use crate::renderer::emulator::global_objects::{GlobalImageId, SamplerInfo};
use crate::renderer::emulator::worker::WorkerTask;

use crate::renderer::emulator::mc_shaders::{McUniformData, ShaderId};
use crate::renderer::emulator::pipeline::{DrawTask, EmulatorOutput, EmulatorPipeline, PipelineTask};
use crate::renderer::emulator::share::Share;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct PassId(u64);

impl PassId {
    pub fn from_raw(id: u64) -> Self {
        Self(id)
    }

    pub fn get_raw(&self) -> u64 {
        self.0
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct ImmediateMeshId(u32);

impl ImmediateMeshId {
    pub fn form_raw(id: u32) -> Self {
        Self(id)
    }

    pub fn get_raw(&self) -> u32 {
        self.0
    }
}

pub struct PassRecorder {
    id: PassId,
    share: Arc<Share>,

    used_shaders: HashSet<ShaderId>,
    used_global_image: HashSet<GlobalImageId>,
    immediate_meshes: Vec<ImmediateMeshInfo>,

    immediate_buffer: Option<Box<ImmediateBuffer>>,

    #[allow(unused)] // We just need to keep the pipeline alive
    pipeline: Arc<dyn EmulatorPipeline>,
}

impl PassRecorder {
    pub(super) fn new(share: Arc<Share>, pipeline: Arc<dyn EmulatorPipeline>, placeholder_image: Arc<GlobalImage>, placeholder_sampler: &SamplerInfo) -> Self {
        let id = share.try_start_pass_id().unwrap_or_else(|| {
            log::error!("Attempted to start pass with an already running pass!");
            panic!();
        });
        let id = PassId::from_raw(id);

        let immediate_buffer = Some(share.get_next_immediate_buffer());

        let placeholder_sampler = placeholder_image.get_sampler(placeholder_sampler);
        share.push_task(WorkerTask::StartPass(id, pipeline.clone(), pipeline.start_pass(), placeholder_image, placeholder_sampler));

        Self {
            id,
            share,

            used_shaders: HashSet::new(),
            used_global_image: HashSet::new(),
            immediate_meshes: Vec::with_capacity(128),

            immediate_buffer,

            pipeline,
        }
    }

    pub fn use_output(&mut self, output: Box<dyn EmulatorOutput + Send>) {
        self.share.push_task(WorkerTask::UseOutput(output));
    }

    pub fn update_uniform(&mut self, data: &McUniformData, shader: ShaderId) {
        self.use_shader(shader);
        self.share.push_task(WorkerTask::PipelineTask(PipelineTask::UpdateUniform(shader, *data)))
    }

    pub fn update_texture(&mut self, index: u32, image: &Arc<GlobalImage>, sampler_info: &SamplerInfo, shader: ShaderId) {
        self.use_shader(shader);
        let view = image.get_sampler_view();
        let sampler = image.get_sampler(sampler_info);

        if self.used_global_image.insert(image.get_id()) {
            self.share.push_task(WorkerTask::UseGlobalImage(image.clone()));
        }

        self.share.push_task(WorkerTask::PipelineTask(PipelineTask::UpdateTexture(shader, index, view, sampler)));
    }

    pub fn upload_immediate(&mut self, data: &MeshData) -> ImmediateMeshId {
        let index_size = data.get_index_size();

        let immediate = self.immediate_buffer.as_mut().unwrap();
        let (vertex_buffer, vertex_offset) = immediate.allocate(data.vertex_data, data.vertex_stride as vk::DeviceSize);
        let (index_buffer, index_offset) = immediate.allocate(data.index_data, index_size as vk::DeviceSize);

        let id = self.immediate_meshes.len() as u32;
        self.immediate_meshes.push(ImmediateMeshInfo {
            vertex_buffer,
            index_buffer,
            vertex_offset: (vertex_offset / (data.vertex_stride as vk::DeviceSize)) as i32,
            first_index: (index_offset / (index_size as vk::DeviceSize)) as u32,
            index_type: data.index_type,
            index_count: data.index_count,
            primitive_topology: data.primitive_topology
        });

        ImmediateMeshId::form_raw(id)
    }

    pub fn draw_immediate(&mut self, id: ImmediateMeshId, shader: ShaderId, depth_write_enable: bool) {
        self.use_shader(shader);

        let mesh_data = self.immediate_meshes.get(id.get_raw() as usize).unwrap();

        let draw_task = DrawTask {
            vertex_buffer: mesh_data.vertex_buffer,
            index_buffer: mesh_data.index_buffer,
            vertex_offset: mesh_data.vertex_offset,
            first_index: mesh_data.first_index,
            index_type: mesh_data.index_type,
            index_count: mesh_data.index_count,
            shader,
            primitive_topology: mesh_data.primitive_topology,
            depth_write_enable,
        };
        self.share.push_task(WorkerTask::PipelineTask(PipelineTask::Draw(draw_task)));
    }

    pub fn draw_global(&mut self, mesh: Arc<GlobalMesh>, shader: ShaderId, depth_write_enable: bool) {
        mesh.update_used_in(self.id);

        self.use_shader(shader);

        let draw_info = mesh.get_draw_info();

        let draw_task = DrawTask {
            vertex_buffer: draw_info.buffer,
            index_buffer: draw_info.buffer,
            vertex_offset: 0,
            first_index: draw_info.first_index,
            index_type: draw_info.index_type,
            index_count: draw_info.index_count,
            shader,
            primitive_topology: draw_info.primitive_topology,
            depth_write_enable,
        };

        self.share.push_task(WorkerTask::UseGlobalMesh(mesh));
        self.share.push_task(WorkerTask::PipelineTask(PipelineTask::Draw(draw_task)));
    }

    fn use_shader(&mut self, shader: ShaderId) {
        if self.used_shaders.insert(shader) {
            self.pipeline.inc_shader_used(shader);
            self.share.push_task(WorkerTask::UseShader(shader));
        }
    }
}

impl Drop for PassRecorder {
    fn drop(&mut self) {
        self.share.push_task(WorkerTask::EndPass(self.immediate_buffer.take().unwrap()));
        self.share.end_pass_id();
    }
}

struct ImmediateMeshInfo {
    vertex_buffer: vk::Buffer,
    index_buffer: vk::Buffer,
    vertex_offset: i32,
    first_index: u32,
    index_type: vk::IndexType,
    index_count: u32,
    primitive_topology: vk::PrimitiveTopology,
}