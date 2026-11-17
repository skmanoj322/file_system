pub enum FileSystemNode {
    File(String),
    Directory(String, Vec<FileSystemNode>),
}

impl FileSystemNode {
    pub fn new() -> Self {
        FileSystemNode::Directory("root".to_string(), vec![])
    }

    pub fn add_file(&mut self, path: &str, filename: String) {
        // ok here I have taken the path as parameter
        // the path should lead to dir i.e parent dir here
        // and starting should start with root here
        //
        // eg /root/dir1/dir1/
        //

        let mut splited_path: Vec<String> = path
            .split('/')
            .map(|part| part.to_string())
            .filter(|part| !part.is_empty())
            .collect(); // just the gut felling like we needed to have the Vec<String>

        // DOUBT
        //
        // we have used filter because if this is the case /root/documents/
        //
        // o/p will be ["","root","document"," "] // I am guessing here
        //

        let parent_dir = Self::find_directory(self, &splited_path).unwrap();

        if let FileSystemNode::Directory(_, dir) = parent_dir {
            dir.push(FileSystemNode::File(filename));
        }
    }

    pub fn add_dir(&mut self, dir_name: String, parent_path: &str) {
        let mut splited_path: Vec<String> = parent_path
            .split('/')
            .map(|part| part.to_string())
            .filter(|part| !part.is_empty())
            .collect();

        let parent_dir = Self::find_directory(self, &splited_path).unwrap();

        if let FileSystemNode::Directory(_, dir) = parent_dir {
            dir.push(FileSystemNode::Directory(dir_name, vec![]));
        }
    }

    // we can add the one more dir here provided parent dir

    // ok here I am
    //
    // no Idea why we have use the static here
    pub fn find_directory<'a>(
        root: &'a mut FileSystemNode,
        path: &[String],
    ) -> Option<&'a mut FileSystemNode> {
        if path.len() == 1 {
            // I am taking the root here and check here like if the path vec contain only one element that is the end of the recurssion here
            // but my concept gap is i want to access the root name here like root is FileSysytem and I want to access the name off the dir here
            //
            // ok accessing the
            if let FileSystemNode::Directory(name, dir) = root {
                if *name == path[0] {
                    return Some(root);
                }
            };
        }

        if let FileSystemNode::Directory(name, dirs) = root {
            if *name == path[0] {
                // I have tried to us the filters here

                for child in dirs {
                    if let Some(found) = Self::find_directory(child, &path[1..]) {
                        return Some(found);
                    }
                }
            }
        }

        None
    }

    pub fn navigate_file_or_dir(name: &str) {} // we can return the  parent
}

// ok here we don't need struct here I guess like
//
// because in new I am providing them the root dir this should be coumulsary here
// next we have to have the add file like it will take the parent_dir as a input to which we need to be created here
// next we can have create the dir here provided the parent_dir here
//
//
// add we also need to have the search algo here which searches the parent_dir where we need to create the file here.
//
