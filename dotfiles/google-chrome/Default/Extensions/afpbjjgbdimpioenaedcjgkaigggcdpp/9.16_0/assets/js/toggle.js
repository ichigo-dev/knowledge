const _KEY = 'toggle_d0k_key';
const _DARK = '0';
const _LIGHT = '1';
const _AUTO = '2';
window.DARK_CSS = '';
// Force dark
var dark_css = '';
function locationCheck(target) {
  return location.href.includes(target);
}
if(locationCheck('options.html') || locationCheck('advanced_features.html')) {
  dark_css = 'assets/css/options.style.dark.css';
}
if(locationCheck('popup.html')) {
  dark_css = 'assets/css/popup.style.dark.css';
}

var dark_css_tag = `<link id="aw_dark_css" rel='stylesheet' href='${dark_css}'/>`;

function writeCSS() {
  document.write(dark_css_tag);
}
var darkQueryList = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)');
if (!darkQueryList) {
  darkQueryList = {
    'matches': false
  }
  darkQueryList.addEventListener = function(){};
}
var st0tus = lsLoadColor();
if(isNaN(st0tus/1)) {
  st0tus = _AUTO;
  lsSaveColor(_AUTO);
}
switch(st0tus) {
  case _DARK:
    writeCSS();
    break;

  case _AUTO:
    if (darkQueryList.matches) {
      writeCSS();
    }
    addWatch();
    break;

  case _LIGHT:
    break;
}

function watchHandler() {
  if(darkQueryList.matches) {
    enterDark();
  } else {
    enterLight();
  }

}
function addWatch() {
  darkQueryList.addEventListener('change',watchHandler);
}
function removeWatch() {
  darkQueryList.removeListener(watchHandler);
}

function lsSaveColor(code) {
  localStorage.setItem(_KEY, code);
}

function lsLoadColor(){
  return localStorage.getItem(_KEY);
}

function enterDark() {
  if($('#aw_dark_css').length == 0) {
    $('head').append(dark_css_tag);
  }
}

function enterLight() {
  $('#aw_dark_css').remove();
}



function toggleButton(code){
  var pick_class = 'color_scheme_pick'
  $('.toggle-sub').removeClass(pick_class)
  switch(code) {
    case _DARK:
      $('.toggle-dark').addClass(pick_class);
      break;

    case _AUTO:
      $('.toggle-auto').addClass(pick_class);
      break;

    case _LIGHT:
      $('.toggle-light').addClass(pick_class);
      break;
  }
}
window.addEventListener('DOMContentLoaded', (e) => {
  toggleButton(st0tus);
  $('body').on('click', '.toggle-dark', ()=>{
    lsSaveColor(_DARK);
    toggleButton(_DARK);
    enterDark();
    removeWatch();
  })
  $('body').on('click', '.toggle-light', ()=>{
    lsSaveColor(_LIGHT);
    toggleButton(_LIGHT);
    enterLight();
    removeWatch();
  })
  $('body').on('click', '.toggle-auto', ()=>{
    lsSaveColor(_AUTO);
    toggleButton(_AUTO);
    addWatch();
    watchHandler();
  })
});
