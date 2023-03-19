var isInDarkMode = false;
function improveDarkMode() {
  try {
    if(window.matchMedia('(prefers-color-scheme: dark)').matches) {
      if(isInDarkMode) {
        return;
      }
      isInDarkMode = true;
      chrome.browserAction.setIcon({
        path:{
          "128":"icon_128_dark.png",
          "48":"icon_48_dark.png",
          "16":"icon_19_dark.png"
        }
      });
    } else {
      if(!isInDarkMode) {
        return;
      }
      setNormalIcon();
    }
  } catch (e){}
}
improveDarkMode();
setInterval(improveDarkMode, 999);
function setNormalIcon(){
  isInDarkMode = false;
  chrome.browserAction.setIcon({
    path:{
      "128":"icon_128.png",
      "48":"icon_48.png",
      "16":"icon_19.png"
    }
  });
}

var MSG_DISABLE_CONTEXT_MENUS = 'A1';
var MSG_ENABLE_CONTEXT_MENUS = 'A2';
var MSG_ENABLE_SCANNER = 'A3';

// Reset all
chrome.contextMenus.removeAll();

chrome.contextMenus.create({
  "title":chrome.i18n.getMessage('share_on_fb'),
  "contexts":["browser_action"],
  "onclick":function(info, tab) {
    chrome.tabs.create({url: 'https://www.facebook.com/sharer/sharer.php?u=https://chrome.google.com/webstore/detail/quick-qr-code-generator/afpbjjgbdimpioenaedcjgkaigggcdpp'});
  }
});

chrome.contextMenus.create({
  "title":chrome.i18n.getMessage('share_on_tw'),
  "contexts":["browser_action"],
  "onclick":function(info, tab) {
    chrome.tabs.create({url: 'https://twitter.com/intent/tweet?text=QR%20Code%20Generator%20for%20Chrome%20%E2%89%AB%20https%3A%2F%2Fchrome.google.com%2Fwebstore%2Fdetail%2Fquick-qr-code-generator%2Fafpbjjgbdimpioenaedcjgkaigggcdpp'});
  }
});

chrome.contextMenus.create({
  "title":"",
  "type":"separator",
  "contexts":["browser_action"],
  "onclick":function(info, tab) {
    chrome.tabs.create({url: 'https://plus.google.com/share?url=https://chrome.google.com/webstore/detail/quick-qr-code-generator/afpbjjgbdimpioenaedcjgkaigggcdpp'});
  }
});

chrome.contextMenus.create({
  "title":"Facebook Page",
  "contexts":["browser_action"],
  "onclick":function(info, tab) {
    chrome.tabs.create({url: 'https://www.facebook.com/quickqr'});
  }
});

createMenus();
function removeMenus(){
  chrome.contextMenus.remove(Number(cmid_text));
  chrome.contextMenus.remove(Number(cmid_link));
  chrome.contextMenus.remove(Number(cmid_page));
}
var cmid_text;
var cmid_link;
var cmid_page;
function createMenus() {
  if(localStorage.getItem('adv-config-context') == '0') return;
  cmid_text = chrome.contextMenus.create({
    "title":chrome.i18n.getMessage("ctx_string"),
    "contexts":["selection"],
    "onclick":function(info, tab) {
    	var target_content = '';
      if (info.selectionText) {
      	target_content = info.selectionText;
      } else {
      	target_content = info.pageUrl;
      }
      chrome.tabs.create({url:"popup.html?c="+target_content});
    }
  });

  cmid_page = chrome.contextMenus.create({
    "title":chrome.i18n.getMessage("ctx_page"),
    "contexts":["page"],
    "onclick":function(info, tab) {
    	var target_content = '';
      if (info.selectionText) {
      	target_content = info.selectionText;
      } else {
      	target_content = info.pageUrl;
      }
      chrome.tabs.create({url:"popup.html?c="+target_content});
    }
  });

  cmid_link = chrome.contextMenus.create({
    "title":chrome.i18n.getMessage("ctx_link"),
    "contexts":["link"],
    "onclick":function(info, tab) {
    	var target_content = '';
      if(info.linkUrl) {
      	target_content = info.linkUrl;
      } else if (info.selectionText) {
      	target_content = info.selectionText;
      } else {
      	target_content = info.pageUrl;
      }
      chrome.tabs.create({url:"popup.html?c="+target_content});
    }
  });
}
if(localStorage.getItem('cmid_scan')) {
  createQRScanner();
}
function createQRScanner(){
  if(window.cmid_scan && localStorage.getItem('cmid_scan')) return;
  window.cmid_scan = chrome.contextMenus.create({
    "title":chrome.i18n.getMessage("ctx_scan"),
    "contexts":["image"]
  });
  localStorage.setItem('cmid_scan', cmid_scan);
}

function is_scanner(cmid) {
  return cmid.toString() == localStorage.getItem('cmid_scan');
}


function toggle_M_pops(){
  
}

chrome.contextMenus.onClicked.addListener(function(info, tab) {
  if(is_scanner(info.menuItemId)) {
    var target_content = '';
    if(info.srcUrl) {
      decoderQR(info.srcUrl);
    } 
  }
});

function decoderQR(src){
  loadImg(src);
  function loadImg(imgsrc) {
    var image = new Image();
    image.src = imgsrc;
    image.onload = function () {
      var width = this.naturalWidth;
      var height = this.naturalHeight;
      createCanvasContext(image, 0, 0, width, height);
    }
  }

  function createCanvasContext(img, t, l, w, h) {
    var canvas = document.createElement('canvas');
    canvas.setAttribute('id', 'qr-canvas');
    canvas.height = h + 100;
    canvas.width = w + 100;
    var context = canvas.getContext('2d');
    context.fillStyle = 'rgb(255,255,255)';
    context.fillRect(0, 0, canvas.width, canvas.height);
    context.drawImage(img, l, t, w, h, 50, 50, w, h);
    var imageData = context.getImageData(0,0,canvas.width, canvas.height);
    const code = jsQR(imageData.data, canvas.width, canvas.height);
    if (code) {
      prompt(chrome.i18n.getMessage("scan_result"),code.data);
    } else {
      alert(chrome.i18n.getMessage("scan_result_fail"));
    }
    // qrcode.callback = read;
    // qrcode.decode(canvas.toDataURL());
  };
}

chrome.runtime.onMessage.addListener(function(request, sender, sendResponse) {
  executeRequest(request.code);
  sendResponse();
  return true;
  // return Promise.resolve("");
});

function executeRequest(code) {
  switch (code) {
    case MSG_DISABLE_CONTEXT_MENUS:
      removeMenus();
      break;
    case MSG_ENABLE_CONTEXT_MENUS:
      createMenus();
      break;
    case MSG_ENABLE_SCANNER:
      createQRScanner();
      break;
  }
}

if(chrome.runtime.setUninstallURL) {
  chrome.runtime.setUninstallURL('https://high-qr-code-generator.com/exit-survey.html');
} else {
  // Not yet enabled
}