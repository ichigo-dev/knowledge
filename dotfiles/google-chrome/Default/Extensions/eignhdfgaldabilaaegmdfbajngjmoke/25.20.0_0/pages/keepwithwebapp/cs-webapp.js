"use strict";!function(){var a,o,s,n,d=window,i=d.location,r=document,e="undefined"!=typeof chrome&&chrome||{};function t(){var e=new UIEvent("resize");d.dispatchEvent(e)}function c(e,t){t=new MouseEvent(t,{bubbles:!0,cancelable:!0,view:window});e.dispatchEvent(t)}function p(){var e=[],t=r.querySelectorAll('body .notes-container div > div:first-child > div > div:first-child > div[role="button"] + div:not([role="button"]) > div[role="button"]'),n=t.length-1;s=t[n];for(var o=0;o<n;o++){var i=t[o],i=i.getAttribute("aria-label")||i.textContent.trim();e.push(i)}d.parent.postMessage({bm_method:"keepwithwebappLabels",bm_value:e},a)}function u(){t(),p()}d.top!==d.self&&"https://keep.google.com"===i.origin&&(a=e.runtime.getURL("PATH/").replace("/PATH/",""),"object"!=typeof(e=i.ancestorOrigins)||1===e.length&&e[0]===a)&&(o=a.startsWith("moz-extension://"),function e(){var t,n,o=r.getElementById("preloaded-theme");if(o)return(t=-1!==i.search.indexOf("bm_udm=1"))&&r.documentElement.classList.add("bm-cs-keepwithwebapp-usedarkmode"),(n=r.head.textContent.match(/https:\/\/www\.gstatic\.com\/_\/apps-notes\/_\/ss\/.+?"/g))?(n=n[t?1:0].replace(/\\u003d/g,"=").split('"')[0],void(o.href!==n&&(o.href=n))):void 0;setTimeout(e,.1)}(),d.addEventListener("message",function(e){var t,n;e.origin===a&&(e.isTrusted||o)&&(t=(e=e.data||{}).bm_method,e=e.bm_value,"keepwithwebappEditLabels"===t?s&&(c(n=s,"mousedown"),c(n,"mouseup"),c(n,"click"),setInterval(p,1e3)):"keepwithwebappSetHash"===t&&(i.hash=e||""))}),function e(){t(),setTimeout(e,1e3)}(),function e(){var t=i.hash.replace("#","")||"home";n!==t&&(d.parent.postMessage({bm_method:"keepwithwebappUpdateView",bm_value:t},a),n=t),setTimeout(e,100)}(),"complete"===r.readyState||"loaded"===r.readyState||"interactive"===r.readyState?u():d.addEventListener("DOMContentLoaded",u))}();