let gb : GitBotInit;
 
fn do_initialize_git()
{
 gb.GitInit("");
}

fn do_add_files(file : string)
{
 gb.GitAddFile(file);
 }
 
 fn add_remote_origin(username : string, respository_name : string)->
 {
    let url_main = "https://github.com/"+username+"/"+respository_name+".git";
    gb.GitRemoteToOrigin(url_main);
 }
 
 fn upload_all_files()
 {
    gb.GitUploadFiles(); // git push origin ..
 }
 
 fn download_files()
 {
   gb.GitDownloadFiles(); // git pull origin ..
 }
    
