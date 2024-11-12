function set_filename_from_upload() {
    let filename_input = document.getElementById("filename")
    if (!filename_input.userChanged) {
        let file_input = document.getElementById("file")
        let filename = file_input.files[0].name.split('.')[0];
        filename_input.value = filename
    }
}