enum class ShaderType(val cliString: String) {
    AUTO(""),
    VERTEX("-fshader-stage=vertex"),
    FRAGMENT("-fshader-stage=fragment"),
    TESSELATION_CONTROL("-fshader-stage=tesscontrol"),
    TESSELATION_EVALUATION("-fshader-stage=tesseval"),
    GEOMETRY("-fshader-stage=geometry"),
    COMPUTE("-fshader-stage=compute"),
}

public abstract class ShaderModule {
    private val name: String;
    public fun getName(): String {
        return this.name;
    }

    @get:Input
    public abstract val source: Property<RelativePath>;

    @get:Input
    public abstract val output: Property<RelativePath>;

    @get:Input
    @Optional
    public val type: Property<ShaderType>;

    @Inject
    constructor(name: String, objectFactory: ObjectFactory) {
        this.name = name;

        this.type = objectFactory.property(ShaderType::class);
        this.type.convention(ShaderType.AUTO);
    }

    public fun source(src: String) {
        this.source(RelativePath(true, src));
    }

    public fun source(src: RelativePath) {
        this.source.set(src);
    }

    public fun output(dst: String) {
        this.output(RelativePath(true, dst));
    }

    public fun output(dst: RelativePath) {
        this.output.set(dst);
    }
}

public abstract class CompileShaders : DefaultTask() {
    @get:Input
    public abstract val baseDir: Property<RelativePath>;

    @get:Input
    public abstract val modules: NamedDomainObjectContainer<ShaderModule>;

    @get:Input
    public abstract val includeDirs: SetProperty<File>;

    @Inject
    protected abstract fun getObjectFactory(): ObjectFactory;

    fun include(dir: Any) {
        this.includeDirs.add(project.file(dir, PathValidation.DIRECTORY));
    }

    fun addModule(name: String, action: Action<in ShaderModule>) {
        this.modules.create(name, action);
    }

    @TaskAction
    fun run() {
        var base = this.baseDir.get();
        var basePath = base.getFile(project.getProjectDir());

        this.modules.forEach {
            var source = base.plus(it.source.get()).getFile(project.getProjectDir());
            var output = base.plus(it.output.get()).getFile(project.getBuildDir());
            var type = it.type.get();
            output.getParentFile().mkdirs();

            project.exec({
                executable("glslc");

                if (type != ShaderType.AUTO) {
                    args(type.cliString);
                }

                args("-I${basePath}");
                
                args("-o${output}");
                args("${source}");
            });
        }
    }
}

tasks.register<CompileShaders>("compileDebugShaders") {
    baseDir.set(RelativePath(false, "debug"));

    addModule("ApplyVert", {
        source("apply.vert")
        output("apply_vert.spv")
    });
    addModule("ApplyFrag", {
        source("apply.frag")
        output("apply_frag.spv")
    });
}