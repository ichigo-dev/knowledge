"use strict";cjce.registerTemplate("bm-p-mybusiness",function(n,s){var a,i,c,t,o,l,d,r,m,e=!cjBasics.webRequest.wrifSupported,b="https://business.google.com",u=b+"/"+s.wizPath,_=s.account.pageId,g=_?"dashboard":"locations",j=!0,p=["dashboard","posts","edit","insights","reviews","messaging","photos","products","jobs","site","users"],h=cjBasics.uniqueId.generate(),w={dashboard:{iconName:"__mdi:dashboard",newTabUrl:!0,brandOnly:!0,label:cjBasics.lang.i18n("cj_i18n_00291","Home")},posts:{iconName:"__mdi:post",newTabUrl:!0,label:cjBasics.lang.i18n("cj_i18n_00990","Posts")},edit:{iconName:"__mdi:store",newTabUrl:!0,label:cjBasics.lang.i18n("cj_i18n_00991","Info")},insights:{iconName:"__mdi:assessment",newTabUrl:!0,label:cjBasics.lang.i18n("cj_i18n_00993","Insights")},reviews:{iconName:"__mdi:rate_review",newTabUrl:!0,label:cjBasics.lang.i18n("cj_i18n_00801","Reviews")},messaging:{iconName:"__mdi:message",newTabUrl:!0,divider:e&&!_,label:cjBasics.lang.i18n("cj_i18n_01727","Messaging")},photos:{iconName:"__mdi:photo_library",newTabUrl:!0,shortcutPathname:"photos",brandOnly:!0,label:cjBasics.lang.i18n("cj_i18n_00994","Photos")},products:{iconName:"__mdi:shopping_basket",newTabUrl:!0,label:cjBasics.lang.i18n("cj_i18n_01823","Products")},jobs:{iconName:"__mdi:list",newTabUrl:!0,label:cjBasics.lang.i18n("cj_i18n_01824","Services")},site:{iconName:"__mdi:web",newTabUrl:!0,label:cjBasics.lang.i18n("cj_i18n_00992","Website"),divider:!0},adwords:{iconName:"__mdi:cjg_ads",newTabUrl:!0,shortcutId:"169",label:cjBasics.lang.i18n("cj_i18n_01501","Create an ad"),external:!e},create:{iconName:"__mdi:add_location",newTabUrl:!0,shortcutPathname:"create",divider:!0,label:cjBasics.lang.i18n("cj_i18n_00555","Add location")},locations:{iconName:"__mdi:business",newTabUrl:!0,label:cjBasics.lang.i18n("cj_i18n_01500","Manage locations")},linkedaccounts:{iconName:"__mdi:link",newTabUrl:!0,label:cjBasics.lang.i18n("cj_i18n_01439","Linked accounts")},settings:{iconName:"__mdi:settings",newTabUrl:!0,label:cjBasics.lang.i18n("cj_i18n_00298","Settings")}};function f(e,n){e=u+e;n.shortcutId&&(e=cjgShortcuts.getUrl(n.shortcutId,s.account)),s.openTab(e)}function v(e){return cjBasics.urlParams.attach(u+e,{bm_cid:"mybusiness",bm_cst:"1",bm_wiz:"1",bm_iid:h,hl:cjBasics.lang.current})}function B(e){e!==j&&(j=e,setTimeout(function(){for(var e=d.children,n=0;n<e.length;n++){var a=e[n];-1!==p.indexOf(a.dataset.id)&&(a.hidden=!j)}d.classList.toggle("bm-ele-navlist--compact",j)},300))}function T(e){r.cjceSetLoading(!0),l=!0,m.src="about:blank",i=!1,-1===p.indexOf(e)&&setTimeout(function(){i||(a.textContent="")},300);var n=v(y(e));setTimeout(function(){l=!1,m.src=n},100)}function y(e){var n=e;return-1!==p.indexOf(e)&&o&&(n+="/l/"+o),n}function C(){r.cjceSetLoading(!1)}function N(e){o=e.listingId,c=e.addressData,t=e.listingTitle;var e=e.viewId;e=e,d.cjceSelectedId=e,r.cjceSetPageLabel(w[e].label),B(-1!==p.indexOf(e)),a.textContent="",i=!0,t&&(e=cjce.createElement("cjmd-title",{label:t}),a.appendChild(e),Array.isArray(c))&&0!==c.length&&((e=cjce.createElement("cjmd-secondarytext",{label:c.join("\n")})).classList.add("bm-p-mybusiness-locationdata__address"),a.appendChild(e))}e?(r=cjce.createElement("bm-ogb",{serviceIcon:"my_business",serviceLabel:cjBasics.lang.i18n("cj_i18n_00554","My Business"),pageLabel:cjBasics.lang.i18n("cj_i18n_01167","Links"),bmApis:s,onNewTab:function(){s.openTab(u+g)}}),n.appendChild(r),e=cjce.createElement("cjmd-container",{scrollable:!0,shadow:"thinOnScroll"}),n.appendChild(e),d=cjce.createElement("bm-navlist",{compact:_,items:(_?["dashboard","posts","edit","insights","reviews","messaging","photos","site","adwords","create","locations","linkedaccounts","settings"]:["posts","reviews","messaging","adwords","create","locations","linkedaccounts","settings"]).map(function(e){var n=w[e];return n.id=e,n}),onClick:f,onNewTab:f}),e.appendChild(d)):(r=cjce.createElement("bm-ogb",{loading:!0,drawer:!0,serviceIcon:"my_business",serviceLabel:cjBasics.lang.i18n("cj_i18n_00554","My Business"),pageLabel:cjBasics.lang.i18n("cj_i18n_01500","Manage locations"),bmApis:s,onNewTab:function(){m.cjceGetCleanUrl(b).then(function(e){s.openTab(e||u+g)})}}),n.appendChild(r),e=r.cjceDrawerEl,(a=document.createElement("div")).className="bm-p-mybusiness-locationdata",e.appendChild(a),d=cjce.createElement("bm-navlist",{compact:!0,isSelector:!0,selectedId:"locations",items:w,onChange:function(e,n){"users"===e?m.cjceSendFrameCommand({method:"bmCstClickDomElement",value:'path[d*="66 0-3 1.34-3 3s1.34 3 3 3zm-8 0c1.66 0 2.99-1.34 2.99-3S9.66 5 8 5C6.34 5 5 6.34 5 8s1.34 3 3 3zm0 2c-2.33 0-7 1.17-7 3.5V19h14v-2.5c0-2.33-4.67-3.5-7-3.5zm8"]'},b):(T(e),e=-1!==p.indexOf(e),r.cjceSetPageLabel(n.label),B(e))},bmApis:s,onNewTab:function(e,n){e="adwords"===e?cjBasics.urlParams.attach("https://adwords.google.com/express/plus/Plus",{__l:o,authuser:"0"===s.account.authuser?null:s.account.authuser}):u+y(e);s.openTab(e)}}),B(!1),e.appendChild(d),cjBasics.webRequest.catchAndStop(["https://accounts.google.com/AccountChooser*service=lbc*"+_+"*dashboard*bm_cid*mybusiness*"],function(){setTimeout(function(){T("locations")})}),cjBasics.webRequest.handleIframeHeaders([b+"/*bm_iid="+h+"*",b+"/local/business/*ih=gmbweb*"],{disableOgs:b,handleFirefoxInject:!0,handleSecVariables:!0}),cjBasics.webRequest.addListener("onCompleted",function(e){399<e.statusCode&&(n.textContent="",e=cjce.createElement("bm-fulldialog",{bmApis:s,onNewTab:function(){s.openTab(b)},lockup:{label:cjBasics.lang.i18n("cj_i18n_00554","My Business")},iconName:"my_business",message:cjBasics.lang.i18n("cj_i18n_06649","Error loading Google My Business. Please confirm if a business is connected to this account."),actionLabel:cjBasics.lang.i18n("cj_i18n_01543","Switch account"),action:function(){s.pageManager.navigate("accountchooser")}}),n.appendChild(e))},{urls:[b+"/*bm_iid="+h+"*"],types:["sub_frame"]}),cjgApis.cache.getItem(s.account,"bm_pref__mybusiness__lastview").then(function(e){"object"!=typeof e||864e5<Date.now()-e.timestamp||(N(e),T(e.viewId))}),m=cjce.createElement("bm-iframe",{src:v(g),solid:"#fff",shadow:!0,onMessage:function(e){var n=e.bm_method,a=e.bm_value;"mybusinessViewData"===n?(N(a),e.timestamp=Date.now(),cjgApis.cache.setItem(s.account,"bm_pref__mybusiness__lastview",a)):"mybusinessDoneLoading"===n&&C()},onLoad:function(){l||C()}}),n.appendChild(m))});