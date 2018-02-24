/*
 * Docker Engine API
 *
 * The Engine API is an HTTP API served by Docker Engine. It is the API the Docker client uses to communicate with the Engine, so everything the Docker client can do can be done with the API.  Most of the client's commands map directly to API endpoints (e.g. `docker ps` is `GET /containers/json`). The notable exception is running containers, which consists of several API calls.  # Errors  The API uses standard HTTP status codes to indicate the success or failure of the API call. The body of the response will be JSON in the following format:  ``` {   \"message\": \"page not found\" } ```  # Versioning  The API is usually changed in each release of Docker, so API calls are versioned to ensure that clients don't break.  For Docker Engine 17.10, the API version is 1.33. To lock to this version, you prefix the URL with `/v1.33`. For example, calling `/info` is the same as calling `/v1.33/info`.  Engine releases in the near future should support this version of the API, so your client will continue to work even if it is talking to a newer Engine.  In previous versions of Docker, it was possible to access the API without providing a version. This behaviour is now deprecated will be removed in a future version of Docker.  If the API version specified in the URL is not supported by the daemon, a HTTP `400 Bad Request` error message is returned.  The API uses an open schema model, which means server may add extra properties to responses. Likewise, the server will ignore any extra query parameters and request body properties. When you write clients, you need to ignore additional properties in responses to ensure they do not break when talking to newer Docker daemons.  This documentation is for version 1.34 of the API. Use this table to find documentation for previous versions of the API:  Docker version  | API version | Changes ----------------|-------------|--------- 17.10.x | [1.33](https://docs.docker.com/engine/api/v1.33/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-33-api-changes) 17.09.x | [1.32](https://docs.docker.com/engine/api/v1.32/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-32-api-changes) 17.07.x | [1.31](https://docs.docker.com/engine/api/v1.31/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-31-api-changes) 17.06.x | [1.30](https://docs.docker.com/engine/api/v1.30/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-30-api-changes) 17.05.x | [1.29](https://docs.docker.com/engine/api/v1.29/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-29-api-changes) 17.04.x | [1.28](https://docs.docker.com/engine/api/v1.28/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-28-api-changes) 17.03.1 | [1.27](https://docs.docker.com/engine/api/v1.27/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-27-api-changes) 1.13.1 & 17.03.0 | [1.26](https://docs.docker.com/engine/api/v1.26/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-26-api-changes) 1.13.0 | [1.25](https://docs.docker.com/engine/api/v1.25/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-25-api-changes) 1.12.x | [1.24](https://docs.docker.com/engine/api/v1.24/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-24-api-changes) 1.11.x | [1.23](https://docs.docker.com/engine/api/v1.23/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-23-api-changes) 1.10.x | [1.22](https://docs.docker.com/engine/api/v1.22/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-22-api-changes) 1.9.x | [1.21](https://docs.docker.com/engine/api/v1.21/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-21-api-changes) 1.8.x | [1.20](https://docs.docker.com/engine/api/v1.20/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-20-api-changes) 1.7.x | [1.19](https://docs.docker.com/engine/api/v1.19/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-19-api-changes) 1.6.x | [1.18](https://docs.docker.com/engine/api/v1.18/) | [API changes](https://docs.docker.com/engine/api/version-history/#v1-18-api-changes)  # Authentication  Authentication for registries is handled client side. The client has to send authentication details to various endpoints that need to communicate with registries, such as `POST /images/(name)/push`. These are sent as `X-Registry-Auth` header as a Base64 encoded (JSON) string with the following structure:  ``` {   \"username\": \"string\",   \"password\": \"string\",   \"email\": \"string\",   \"serveraddress\": \"string\" } ```  The `serveraddress` is a domain/IP without a protocol. Throughout this structure, double quotes are required.  If you have already got an identity token from the [`/auth` endpoint](#operation/SystemAuth), you can just pass this instead of credentials:  ``` {   \"identitytoken\": \"9cbaf023786cd7...\" } ```
 *
 * OpenAPI spec version: 1.34
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerCreateBody {
    /// The hostname to use for the container, as a valid RFC 1123 hostname.
    #[serde(rename = "Hostname")]
    hostname: Option<String>,
    /// The domain name to use for the container.
    #[serde(rename = "Domainname")]
    domainname: Option<String>,
    /// The user that commands are run as inside the container.
    #[serde(rename = "User")]
    user: Option<String>,
    /// Whether to attach to `stdin`.
    #[serde(rename = "AttachStdin")]
    attach_stdin: Option<bool>,
    /// Whether to attach to `stdout`.
    #[serde(rename = "AttachStdout")]
    attach_stdout: Option<bool>,
    /// Whether to attach to `stderr`.
    #[serde(rename = "AttachStderr")]
    attach_stderr: Option<bool>,
    /// An object mapping ports to an empty object in the form:  `{\"<port>/<tcp|udp>\": {}}`
    #[serde(rename = "ExposedPorts")]
    exposed_ports: Option<::std::collections::HashMap<String, Value>>,
    /// Attach standard streams to a TTY, including `stdin` if it is not closed.
    #[serde(rename = "Tty")]
    tty: Option<bool>,
    /// Open `stdin`
    #[serde(rename = "OpenStdin")]
    open_stdin: Option<bool>,
    /// Close `stdin` after one attached client disconnects
    #[serde(rename = "StdinOnce")]
    stdin_once: Option<bool>,
    /// A list of environment variables to set inside the container in the form `[\"VAR=value\", ...]`. A variable without `=` is removed from the environment, rather than to have an empty value.
    #[serde(rename = "Env")]
    env: Option<Vec<String>>,
    #[serde(rename = "Healthcheck")]
    healthcheck: Option<::models::HealthConfig>,
    /// Command is already escaped (Windows only)
    #[serde(rename = "ArgsEscaped")]
    args_escaped: Option<bool>,
    /// The name of the image to use when creating the container
    #[serde(rename = "Image")]
    image: Option<String>,
    #[serde(rename = "Volumes")]
    volumes: Option<::models::ContainerConfigVolumes>,
    /// The working directory for commands to run in.
    #[serde(rename = "WorkingDir")]
    working_dir: Option<String>,
    /// Disable networking for the container.
    #[serde(rename = "NetworkDisabled")]
    network_disabled: Option<bool>,
    /// MAC address of the container.
    #[serde(rename = "MacAddress")]
    mac_address: Option<String>,
    /// `ONBUILD` metadata that were defined in the image's `Dockerfile`.
    #[serde(rename = "OnBuild")]
    on_build: Option<Vec<String>>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    labels: Option<::std::collections::HashMap<String, String>>,
    /// Signal to stop a container as a string or unsigned integer.
    #[serde(rename = "StopSignal")]
    stop_signal: Option<String>,
    /// Timeout to stop a container in seconds.
    #[serde(rename = "StopTimeout")]
    stop_timeout: Option<i32>,
    /// Shell for when `RUN`, `CMD`, and `ENTRYPOINT` uses a shell.
    #[serde(rename = "Shell")]
    shell: Option<Vec<String>>,
    #[serde(rename = "HostConfig")]
    host_config: Option<::models::HostConfig>,
    #[serde(rename = "NetworkingConfig")]
    networking_config: Option<::models::ContainerCreateBodyNetworkingConfig>,
}

impl ContainerCreateBody {
    pub fn new() -> ContainerCreateBody {
        ContainerCreateBody {
            hostname: None,
            domainname: None,
            user: None,
            attach_stdin: None,
            attach_stdout: None,
            attach_stderr: None,
            exposed_ports: None,
            tty: None,
            open_stdin: None,
            stdin_once: None,
            env: None,
            healthcheck: None,
            args_escaped: None,
            image: None,
            volumes: None,
            working_dir: None,
            network_disabled: None,
            mac_address: None,
            on_build: None,
            labels: None,
            stop_signal: None,
            stop_timeout: None,
            shell: None,
            host_config: None,
            networking_config: None,
        }
    }

    pub fn set_hostname(&mut self, hostname: String) {
        self.hostname = Some(hostname);
    }

    pub fn with_hostname(mut self, hostname: String) -> ContainerCreateBody {
        self.hostname = Some(hostname);
        self
    }

    pub fn hostname(&self) -> Option<&String> {
        self.hostname.as_ref()
    }

    pub fn reset_hostname(&mut self) {
        self.hostname = None;
    }

    pub fn set_domainname(&mut self, domainname: String) {
        self.domainname = Some(domainname);
    }

    pub fn with_domainname(mut self, domainname: String) -> ContainerCreateBody {
        self.domainname = Some(domainname);
        self
    }

    pub fn domainname(&self) -> Option<&String> {
        self.domainname.as_ref()
    }

    pub fn reset_domainname(&mut self) {
        self.domainname = None;
    }

    pub fn set_user(&mut self, user: String) {
        self.user = Some(user);
    }

    pub fn with_user(mut self, user: String) -> ContainerCreateBody {
        self.user = Some(user);
        self
    }

    pub fn user(&self) -> Option<&String> {
        self.user.as_ref()
    }

    pub fn reset_user(&mut self) {
        self.user = None;
    }

    pub fn set_attach_stdin(&mut self, attach_stdin: bool) {
        self.attach_stdin = Some(attach_stdin);
    }

    pub fn with_attach_stdin(mut self, attach_stdin: bool) -> ContainerCreateBody {
        self.attach_stdin = Some(attach_stdin);
        self
    }

    pub fn attach_stdin(&self) -> Option<&bool> {
        self.attach_stdin.as_ref()
    }

    pub fn reset_attach_stdin(&mut self) {
        self.attach_stdin = None;
    }

    pub fn set_attach_stdout(&mut self, attach_stdout: bool) {
        self.attach_stdout = Some(attach_stdout);
    }

    pub fn with_attach_stdout(mut self, attach_stdout: bool) -> ContainerCreateBody {
        self.attach_stdout = Some(attach_stdout);
        self
    }

    pub fn attach_stdout(&self) -> Option<&bool> {
        self.attach_stdout.as_ref()
    }

    pub fn reset_attach_stdout(&mut self) {
        self.attach_stdout = None;
    }

    pub fn set_attach_stderr(&mut self, attach_stderr: bool) {
        self.attach_stderr = Some(attach_stderr);
    }

    pub fn with_attach_stderr(mut self, attach_stderr: bool) -> ContainerCreateBody {
        self.attach_stderr = Some(attach_stderr);
        self
    }

    pub fn attach_stderr(&self) -> Option<&bool> {
        self.attach_stderr.as_ref()
    }

    pub fn reset_attach_stderr(&mut self) {
        self.attach_stderr = None;
    }

    pub fn set_exposed_ports(&mut self, exposed_ports: ::std::collections::HashMap<String, Value>) {
        self.exposed_ports = Some(exposed_ports);
    }

    pub fn with_exposed_ports(
        mut self,
        exposed_ports: ::std::collections::HashMap<String, Value>,
    ) -> ContainerCreateBody {
        self.exposed_ports = Some(exposed_ports);
        self
    }

    pub fn exposed_ports(&self) -> Option<&::std::collections::HashMap<String, Value>> {
        self.exposed_ports.as_ref()
    }

    pub fn reset_exposed_ports(&mut self) {
        self.exposed_ports = None;
    }

    pub fn set_tty(&mut self, tty: bool) {
        self.tty = Some(tty);
    }

    pub fn with_tty(mut self, tty: bool) -> ContainerCreateBody {
        self.tty = Some(tty);
        self
    }

    pub fn tty(&self) -> Option<&bool> {
        self.tty.as_ref()
    }

    pub fn reset_tty(&mut self) {
        self.tty = None;
    }

    pub fn set_open_stdin(&mut self, open_stdin: bool) {
        self.open_stdin = Some(open_stdin);
    }

    pub fn with_open_stdin(mut self, open_stdin: bool) -> ContainerCreateBody {
        self.open_stdin = Some(open_stdin);
        self
    }

    pub fn open_stdin(&self) -> Option<&bool> {
        self.open_stdin.as_ref()
    }

    pub fn reset_open_stdin(&mut self) {
        self.open_stdin = None;
    }

    pub fn set_stdin_once(&mut self, stdin_once: bool) {
        self.stdin_once = Some(stdin_once);
    }

    pub fn with_stdin_once(mut self, stdin_once: bool) -> ContainerCreateBody {
        self.stdin_once = Some(stdin_once);
        self
    }

    pub fn stdin_once(&self) -> Option<&bool> {
        self.stdin_once.as_ref()
    }

    pub fn reset_stdin_once(&mut self) {
        self.stdin_once = None;
    }

    pub fn set_env(&mut self, env: Vec<String>) {
        self.env = Some(env);
    }

    pub fn with_env(mut self, env: Vec<String>) -> ContainerCreateBody {
        self.env = Some(env);
        self
    }

    pub fn env(&self) -> Option<&Vec<String>> {
        self.env.as_ref()
    }

    pub fn reset_env(&mut self) {
        self.env = None;
    }

    pub fn set_healthcheck(&mut self, healthcheck: ::models::HealthConfig) {
        self.healthcheck = Some(healthcheck);
    }

    pub fn with_healthcheck(mut self, healthcheck: ::models::HealthConfig) -> ContainerCreateBody {
        self.healthcheck = Some(healthcheck);
        self
    }

    pub fn healthcheck(&self) -> Option<&::models::HealthConfig> {
        self.healthcheck.as_ref()
    }

    pub fn reset_healthcheck(&mut self) {
        self.healthcheck = None;
    }

    pub fn set_args_escaped(&mut self, args_escaped: bool) {
        self.args_escaped = Some(args_escaped);
    }

    pub fn with_args_escaped(mut self, args_escaped: bool) -> ContainerCreateBody {
        self.args_escaped = Some(args_escaped);
        self
    }

    pub fn args_escaped(&self) -> Option<&bool> {
        self.args_escaped.as_ref()
    }

    pub fn reset_args_escaped(&mut self) {
        self.args_escaped = None;
    }

    pub fn set_image(&mut self, image: String) {
        self.image = Some(image);
    }

    pub fn with_image(mut self, image: String) -> ContainerCreateBody {
        self.image = Some(image);
        self
    }

    pub fn image(&self) -> Option<&String> {
        self.image.as_ref()
    }

    pub fn reset_image(&mut self) {
        self.image = None;
    }

    pub fn set_volumes(&mut self, volumes: ::models::ContainerConfigVolumes) {
        self.volumes = Some(volumes);
    }

    pub fn with_volumes(
        mut self,
        volumes: ::models::ContainerConfigVolumes,
    ) -> ContainerCreateBody {
        self.volumes = Some(volumes);
        self
    }

    pub fn volumes(&self) -> Option<&::models::ContainerConfigVolumes> {
        self.volumes.as_ref()
    }

    pub fn reset_volumes(&mut self) {
        self.volumes = None;
    }

    pub fn set_working_dir(&mut self, working_dir: String) {
        self.working_dir = Some(working_dir);
    }

    pub fn with_working_dir(mut self, working_dir: String) -> ContainerCreateBody {
        self.working_dir = Some(working_dir);
        self
    }

    pub fn working_dir(&self) -> Option<&String> {
        self.working_dir.as_ref()
    }

    pub fn reset_working_dir(&mut self) {
        self.working_dir = None;
    }

    pub fn set_network_disabled(&mut self, network_disabled: bool) {
        self.network_disabled = Some(network_disabled);
    }

    pub fn with_network_disabled(mut self, network_disabled: bool) -> ContainerCreateBody {
        self.network_disabled = Some(network_disabled);
        self
    }

    pub fn network_disabled(&self) -> Option<&bool> {
        self.network_disabled.as_ref()
    }

    pub fn reset_network_disabled(&mut self) {
        self.network_disabled = None;
    }

    pub fn set_mac_address(&mut self, mac_address: String) {
        self.mac_address = Some(mac_address);
    }

    pub fn with_mac_address(mut self, mac_address: String) -> ContainerCreateBody {
        self.mac_address = Some(mac_address);
        self
    }

    pub fn mac_address(&self) -> Option<&String> {
        self.mac_address.as_ref()
    }

    pub fn reset_mac_address(&mut self) {
        self.mac_address = None;
    }

    pub fn set_on_build(&mut self, on_build: Vec<String>) {
        self.on_build = Some(on_build);
    }

    pub fn with_on_build(mut self, on_build: Vec<String>) -> ContainerCreateBody {
        self.on_build = Some(on_build);
        self
    }

    pub fn on_build(&self) -> Option<&Vec<String>> {
        self.on_build.as_ref()
    }

    pub fn reset_on_build(&mut self) {
        self.on_build = None;
    }

    pub fn set_labels(&mut self, labels: ::std::collections::HashMap<String, String>) {
        self.labels = Some(labels);
    }

    pub fn with_labels(
        mut self,
        labels: ::std::collections::HashMap<String, String>,
    ) -> ContainerCreateBody {
        self.labels = Some(labels);
        self
    }

    pub fn labels(&self) -> Option<&::std::collections::HashMap<String, String>> {
        self.labels.as_ref()
    }

    pub fn reset_labels(&mut self) {
        self.labels = None;
    }

    pub fn set_stop_signal(&mut self, stop_signal: String) {
        self.stop_signal = Some(stop_signal);
    }

    pub fn with_stop_signal(mut self, stop_signal: String) -> ContainerCreateBody {
        self.stop_signal = Some(stop_signal);
        self
    }

    pub fn stop_signal(&self) -> Option<&String> {
        self.stop_signal.as_ref()
    }

    pub fn reset_stop_signal(&mut self) {
        self.stop_signal = None;
    }

    pub fn set_stop_timeout(&mut self, stop_timeout: i32) {
        self.stop_timeout = Some(stop_timeout);
    }

    pub fn with_stop_timeout(mut self, stop_timeout: i32) -> ContainerCreateBody {
        self.stop_timeout = Some(stop_timeout);
        self
    }

    pub fn stop_timeout(&self) -> Option<&i32> {
        self.stop_timeout.as_ref()
    }

    pub fn reset_stop_timeout(&mut self) {
        self.stop_timeout = None;
    }

    pub fn set_shell(&mut self, shell: Vec<String>) {
        self.shell = Some(shell);
    }

    pub fn with_shell(mut self, shell: Vec<String>) -> ContainerCreateBody {
        self.shell = Some(shell);
        self
    }

    pub fn shell(&self) -> Option<&Vec<String>> {
        self.shell.as_ref()
    }

    pub fn reset_shell(&mut self) {
        self.shell = None;
    }

    pub fn set_host_config(&mut self, host_config: ::models::HostConfig) {
        self.host_config = Some(host_config);
    }

    pub fn with_host_config(mut self, host_config: ::models::HostConfig) -> ContainerCreateBody {
        self.host_config = Some(host_config);
        self
    }

    pub fn host_config(&self) -> Option<&::models::HostConfig> {
        self.host_config.as_ref()
    }

    pub fn reset_host_config(&mut self) {
        self.host_config = None;
    }

    pub fn set_networking_config(
        &mut self,
        networking_config: ::models::ContainerCreateBodyNetworkingConfig,
    ) {
        self.networking_config = Some(networking_config);
    }

    pub fn with_networking_config(
        mut self,
        networking_config: ::models::ContainerCreateBodyNetworkingConfig,
    ) -> ContainerCreateBody {
        self.networking_config = Some(networking_config);
        self
    }

    pub fn networking_config(&self) -> Option<&::models::ContainerCreateBodyNetworkingConfig> {
        self.networking_config.as_ref()
    }

    pub fn reset_networking_config(&mut self) {
        self.networking_config = None;
    }
}