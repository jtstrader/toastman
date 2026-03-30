use std::collections::HashMap;

/// The main application state.
#[derive(Debug)]
pub struct App {
    /// A series of top-level folders (workspaces) that are isolated and have their own
    /// environments.
    workspaces: Vec<Workspace>,
}

#[derive(Debug)]
pub struct Workspace {
    /// The environment that applies to all subfolders/requests of the workspace.
    environment: HashMap<String, String>,

    /// The entry point of the workspace.
    root: Folder,
}

/// A folder is a named container that can have requests or more folders within.
#[derive(Debug)]
pub struct Folder {
    /// The name of the folder.
    name: String,

    /// Subfolders that can contain additional subfolders/requests.
    folders: HashMap<u16, Folder>,

    /// A series of requests associated with this folder.
    requests: Vec<Request>,
}

/// A request object.
#[derive(Debug)]
pub struct Request {
    method: RequestMethod,
    base_url: String,
    endpoint: String,
    query_params: HashMap<String, String>,
    path_params: HashMap<String, String>,
}

/// An HTTP request method.
#[derive(Debug)]
pub enum RequestMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}
