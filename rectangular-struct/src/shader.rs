fn shader_from_source(source: &CStr, kind: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
  unsafe { let id = gl::CreateShader(kind); gl::ShaderSource(id, 1, &source.as_ptr(), 
  std::ptr::null()); gl::CompileShader(id); let mut success = 1; gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
  if success == 0 { let mut len = 0; gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
  let mut buffer = Vec::with_capacity(len as usize); gl::GetShaderInfoLog(id, len, ptr::null_mut(), buffer.as_mut_ptr());
  buffer.set_len(len as usize); Err(String::from_utf8_unchecked(buffer))} else { Ok(id) } } struct Rect { width: u32,
  height: u32, } impl Rect { fn find_area(&self) -> u32 { self.width * self.height }
    fn can_fit(&self, other: &Rect) -> bool { self.width > other.width && self.height > other.height }
    fn can_fit_rotated(&self, other: &Rect) -> bool { if self.width > other.width && self.height > other.height { true
      } else if self.width > other.height && self.height > other.width {true} else {false}}} let rect1 = Rect { width: 30,
    height: 50, }; let rect2 = Rect { width: 40, height: 20, }; // let rect3 = Rect { width: 10, height: 5}; 
  println!("{}", rect1.find_area()); println!("{}", rect1.can_fit(&rect2)); println!("{}", rect1.can_fit_rotated(&rect2))
  pub use crate::Compiler; pub struct Builder<'a> { pub build: &'a Build, pub top_stage: u32, pub kind: Kind, 
  cache: Cache, stack: RefCell<Vec<Box<dyn Any>>>, time_spent_on_dependencies: Cell<Duration>,
 pub paths: Vec<PathBuf>, }  impl<'a> Deref for Builder<'a> { type Target = Build; 
  fn deref(&self) -> &Self::Target { self.build }}
 pub struct RunConfig<'a> { pub builder: &'a Builder<'a>, pub target: TargetSelection, pub path: PathBuf, }
impl RunConfig<'_> { pub fn build_triple(&self) -> TargetSelection { self.builder.build.build }}
struct StepDescription { defaonly_hosts: bool, should_run: 
  fn(ShouldRun<'_>) -> ShouldRun<'_>, make_run: fn(RunConfig<'_>), name: &'static str,} impl PathSet {
  fn empty() -> PathSet { PathSet::Set(BTreeSet::new())} fn one<P: Into<PathBuf>>(path: P) -> PathSet {
  let mut set = BTreeSet::new(); set.insert(path.into()); PathSet::Set(set)}
  fn has(&self, needle: &Path) -> bool { match self { PathSet::Set(set) => 
    set.iter().any(|p| p.ends_with(needle)), PathSet::Suite(suite) => suite.ends_with(needle), 
 } } fn path(&self, builder: &Builder<'_>) -> PathBuf { match self { PathSet::Set(set)
  => set.iter().next().unwrap_or(&builder.build.src).to_path_buf(), PathSet::Suite(path) => PathBuf::from(path),
 }}} impl StepDescription { fn from<S: Step>() -> StepDescription { StepDescription {default: 
  S::DEFAULT,only_hosts: S::ONLY_HOSTS,should_run: S::should_run,make_run: S::make_run,name: std::any::type_name::<S>(),
}}  fn maybe_run(&self, builder: &Builder<'_>, pathset: &PathSet) { if builder.config.exclude.iter().any(|e| pathset.has(e))
 { eprintln!("Skipping {:?} because it is excluded", pathset); return; } 
 else if !builder.config.exclude.is_empty() { 
eprintln!( "{:?} not skipped for {:?} -- not in {:?}", pathset, self.name, builder.config.exclude);} 
let targets = if self.only_hosts { &builder.hosts } else { &builder.targets }; for target in targets {
let run = RunConfig { builder, path: pathset.path(builder), target: *target };
(self.make_run)(run);}} fn run(v: &[StepDescription], builder: &Builder<'_>, paths: &[PathBuf]) { let should_runs =
v.iter().map(|desc| (desc.should_run)(ShouldRun::new(builder))).collect::<Vec<_>>(); for (desc, should_run) in v.iter().zip(&should_runs) { assert!(
!should_run.paths.is_empty(), "{:?} should have at least one pathset", desc.name ); }
if paths.is_empty() || builder.config.include_default_paths { for (desc, should_run) in v.iter().zip(&should_runs) {
  if desc.default && should_run.is_really_default { for pathset in &should_run.paths {
    desc.maybe_run(builder, pathset); }} }} for path in paths {
          let path = match path.strip_prefix(".") {
              Ok(p) => p, Err(_) => path, }; let mut attempted_run = false; for (desc, should_run) in v.iter().zip(&should_runs) {
              if let Some(suite) = should_run.is_suite_path(path) { attempted_run = true; desc.maybe_run(builder, suite);
              } else if let Some(pathset) = should_run.pathset_for_path(path) { attempted_run = true; desc.maybe_run(builder, pathset);
              } } if !attempted_run { panic!("error: no rules matched {}", path.display());
  }}}}


}




