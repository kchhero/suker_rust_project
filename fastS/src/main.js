const { invoke } = window.__TAURI__.core;

import { isPathIsFile, toastOn, splitFileName } from "./common.js";

let coll = document.getElementsByClassName("collapsible");

let inputSearchMain = document.querySelector("#id-input-search-main");
let inputSearchExt = document.querySelector("#id-input-search-ext");
let inputCheckHidden = document.querySelector("#id-check-hidden");
let inputCheckStrict = document.querySelector("#id-check-strict");
let inputCheckDialog = document.querySelector("#id-check-opendialog");
let inputLocation = document.querySelector("#id-input-location");
let inputLocationMore1 = document.querySelector("#id-input-more1");
let inputLocationMore2 = document.querySelector("#id-input-more2");
let inputLocationMore3 = document.querySelector("#id-input-more3");

let inputDepth = document.querySelector("#id-input-depth");
let inputLimit = document.querySelector("#id-input-limit");

let resultOut = document.querySelector("#id-select-out");
let drawCircle = document.querySelector("#id-drawcircle");

let timerId_check;

var bSearchDone = false;
var bDisplayDone = false;

//-------------------------------------------------------------------
//initialize
//-------------------------------------------------------------------
function init() {
  let i;

  for (i = 0; i < coll.length; i++) {
    coll[i].addEventListener("click", function() {
      this.classList.toggle("active");
      var foldcontent = this.nextElementSibling;
      if (foldcontent.style.display === "block") {
        foldcontent.style.display = "none";
      } else {
        foldcontent.style.display = "block";
      }
    });
  }

  //default setup
  inputSearchExt.disabled = false;
  inputSearchMain.textContent = "";
  inputSearchExt.textContent = "";
  inputCheckHidden.checked = true;
  inputCheckStrict.checked = true;
  inputCheckDialog.checked = true;
  inputLocation.textContent = "";
  inputLocationMore1.textContent = "";
  inputLocationMore2.textContent = "";
  inputLocationMore3.textContent = "";
  inputDepth.value = "100";
  inputLimit.value = "100";

  drawCircle.style.display = "none";

  bDisplayDone = false;
}

init();
//-------------------------------------------------------------------

async function run() {
  bSearchDone = false;
  bDisplayDone = false;

  let more = [];

  if (inputLocationMore1.value.length > 0) {
    more.push(inputLocationMore1.value);
  }
  if (inputLocationMore2.value.length > 0) {
    more.push(inputLocationMore2.value);
  }
  if (inputLocationMore3.value.length > 0) {
    more.push(inputLocationMore3.value);
  }

  var callbackName = "do_search_file";
  let tempExt;
  let tempMain;

  if (inputSearchExt.disabled == true) {
    var temp = splitFileName(inputSearchMain.value);
    tempMain = temp[0];
    tempExt = temp[1];
  } else {
    tempMain = inputSearchMain.value;
    tempExt = inputSearchExt.value;
    if (tempExt.length == 0) { //this is a folder
      callbackName = "do_search_dir";
      changeHeaderText("searching directory...");
    }
    else {
      changeHeaderText("searching files...");
    }
  }  

  drawCircle.style.display = "block";
  timerId_check = setInterval(() => checkDone(), 1000);

  await invoke(callbackName, { 
    locStart: inputLocation.value, inSearch: tempMain, locMore: more,    
    limit: Number(inputLimit.value), ext: tempExt,
    depth: Number(inputDepth.value), hidden: inputCheckHidden.checked, strict: inputCheckStrict.checked
  });
}

async function checkDone() {
  if (bSearchDone == false) {
    if (true == await invoke("rust_check_search_done", {})) {
      clearInterval(timerId_check);
      bSearchDone = true;
      displayList();
      changeHeaderText("What are you looking for?");
    }
  }	
}

async function displayList() {
  if (bDisplayDone == false) {
    resultOut.options.length = 0;
    drawCircle.style.display = "none";  

    let v = await invoke("rust_make_vec_result", {});
    console.log(v.length);
    for (let i = 0; i < v.length; i++) {
      console.log(v[i]);
      
      var option = document.createElement("option");
      option.innerText = v[i];
      resultOut.appendChild(option);
    }
    bDisplayDone = true;
  }
}

async function setDialog(id) {
  let v = await invoke("rust_open_dir_dialog", {});
  if (id == 1) {
    inputLocationMore1.value = v;    
    console.log(v);
  } else if (id == 2) {
    inputLocationMore2.value = v;
    console.log(v);
  } else if (id == 3) {
    inputLocationMore3.value = v;
    console.log(v);
  } else { //0
    inputLocation.value = v;
    console.log(v);
  }
}

function inputTypeNumber(event) {
  //check string or number
  var result = event.target.value;
  if(isNaN(result)) { //this is a string
    event.target.textContent = "";
  } else{
    toastOn("Please input type number!", 2);
  }
}

window.addEventListener("DOMContentLoaded", () => {
  var focusableItems;
  var focusableItems1 = document.querySelectorAll('.focusableInput');
  var focusableItems2 = document.querySelectorAll('.focusable');
  let currentFocusIndex = 0;

  // Set initial focus
  focusableItems = [...focusableItems1, ...focusableItems2];
  focusableItems[currentFocusIndex].focus();

  var btnRun = document.getElementById('id-button-run');
  var btnLocation = document.getElementById('id-button-location');  
  var btnLocationMore1 = document.getElementById('id-button-more1');
  var btnLocationMore2 = document.getElementById('id-button-more2');
  var btnLocationMore3 = document.getElementById('id-button-more3');

  function btn_listener(event) {
    switch(event.target.id){
      case 'id-button-location':       setDialog(0);       break;
      case 'id-button-more1':          setDialog(1);       break;
      case 'id-button-more2':          setDialog(2);       break;
      case 'id-button-more3':          setDialog(3);       break;
      case 'id-button-run':			 run();				    break;          
    }
  }
  btnRun.addEventListener('click', btn_listener);
  btnLocation.addEventListener('click', btn_listener);
  btnLocationMore1.addEventListener('click', btn_listener);
  btnLocationMore2.addEventListener('click', btn_listener);
  btnLocationMore3.addEventListener('click', btn_listener); 

  inputSearchMain.addEventListener("focusout", (event) => {
    if (isPathIsFile(event.target.value)) {
    //this is a file, extension field is disabled
    inputSearchExt.disabled = true;
    } else {
    //this is a folder
    inputSearchExt.disabled = false;
    }
  });

  inputLimit.addEventListener("focusout", (event) => {
    inputTypeNumber(event);
  });
  inputDepth.addEventListener("focusout", (event) => {
    inputTypeNumber(event);
  });

  document.addEventListener('keydown', (event) => {
    if (event.key === 'Tab') {
      event.preventDefault();
      // Remove focus from the currently focused button
      focusableItems[currentFocusIndex].blur();
      // Update the current focus index
      if (event.shiftKey) {
        // Shift + Tab moves focus backward
        currentFocusIndex = (currentFocusIndex - 1 + focusableItems.length) % focusableItems.length;
      } else {
        // Tab moves focus forward
        currentFocusIndex = (currentFocusIndex + 1) % focusableItems.length;
      }
      // Set focus to the new button
      focusableItems[currentFocusIndex].focus();
    }
    // Handle Enter key press for "click" action
    if (event.key === 'Enter') {
      if (document.activeElement.classList.contains('custom-button')) {
        document.activeElement.click();
      }
    }
  });
});

resultOut.addEventListener("dblclick", (event) => {
  if (resultOut.options.length > 0) {
    var result = event.target.value;
    console.log(result);
  //open dialog
  if (inputCheckDialog.checked == true) {
    invoke("rust_open_dir_dialog_standalone", { path: result });
    toastOn("Open Explorer...", 2); //2 seconds
  } else {
    //copy to clipboard
    navigator.clipboard.writeText(result).then(function() {
    console.log('Async: Copying to clipboard was successful!');
    toastOn("Clipboard copyed OK !", 2);
  }, function(err) {
      console.error('Async: Could not copy text: ', err);
      toastOn("Clipboard copyed FAILED !", 2);
    });
  }
}
});

function changeHeaderText(newText) {	
  if (headerText !== null) {
    document.getElementById("headerText").innerText = newText;
  }
}