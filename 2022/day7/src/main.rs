use std::fs;
struct File {
    file_name: String,
    file_size: u32
}

struct Folder {
    name: String,
    parent_folder: Option<Box<&Folder>>,
    child_folders: Vec<Folder>,
    files: Vec<File>,
}

impl Folder {
    fn add_child_folder(&self,folder_name:&str){
        let parent_folder: Option<Box<&Folder>> = Some(Box::new(self));
        let new_folder = Folder{name:folder_name.to_string(),parent_folder:parent_folder,child_folders:Vec::new(),files:Vec::new()};
        self.child_folders.push(new_folder);
    }
    fn add_file(&self, file_name:&str,file_size:u32) {
        let new_file: File = File{file_name:file_name.to_string(),file_size: file_size};
        self.files.push(new_file);
    }

    fn cd_to_child_folder(self, folder_name:&str) -> &Folder {
        for child_folder in self.child_folders {
            if child_folder.name == folder_name {
                return &child_folder;
            }
        }
        panic!("Could not cd to child")
    }

    fn cd_to_parent_folder(self) -> Option<Box<&Folder>> {
        self.parent_folder
    }
}

fn change_directory(mut current_folder:&Folder,folder_name:&str) {
    if folder_name == ".." {
        current_folder = match current_folder.parent_folder {
            Some(folder) => {&folder},
            // TODO
            None => {panic!("Cannot go to parent already at root directory.")},
        }
    }
    else {
        current_folder = current_folder.cd_to_child_folder(folder_name);
    }

}


fn main() {
    println!("Hello, world!");
    let content:String = fs::read_to_string("src/test.txt").expect("Could not read file");
    for instruction in content.split("\n"){
        println!("{instruction}");
    }
    print!("{content}");

    let root_dir:Folder = Folder{name:"/".to_string(),parent_folder: None,child_folders: Vec::new(),files: Vec::new()};
    let current_folder: &Folder = &root_dir;
    for command_str in content.split("\n") {
        let command_parts: Vec<&str> = command_str.split(" ").collect();

        if command_str.starts_with("$ cd") {
            change_directory(current_folder, command_parts[2])
        }
        else if command_str.starts_with("$ ls") {
            
        }
        else if command_str.starts_with("dir "){
            current_folder.add_child_folder(command_parts[1])
        }
        else if command_str[0..1].parse::<u8>().is_ok() {
            current_folder.add_file(command_parts[0], command_parts[1].parse().unwrap())
        }
        else {
            panic!("Could not recognize input {command_str}");
        }
}

}
