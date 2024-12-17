export function isPathIsFile(path) {
    var result = path.slice(-4,);
    if(result[0] == ".") {
        //this is a file, extension field is disabled
        console.log("this is a file");
        return true;
    }
    else{
        //this is a folder
        console.log("this is a folder");
        return false;
    }
}
    
export function splitFileName(path) {
    var result = path.slice(-4,);
    var ret = [];
    //file name save in ret[0]
    //extension save in ret[1]
    if(result[0] == ".") {
        //this is a file, extension field is disabled
        ret[0] = path.slice(0,-4);
        ret[1] = path.slice(-3,);
    }
    else{
        //this is a folder
        ret[0] = path;
        ret[1] = "";
    }
    return ret;
}
    
export function toastOn(msg, duration){
    var toastMessage = document.getElementById('id-toast');
    toastMessage.innerHTML = msg;
    toastMessage.classList.add('active');
    setTimeout(function(){
        toastMessage.classList.remove('active');
    },duration*1000);
}