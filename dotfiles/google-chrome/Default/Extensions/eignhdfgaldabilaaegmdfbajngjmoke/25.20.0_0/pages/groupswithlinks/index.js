"use strict";cjce.registerTemplate("bm-p-groupswithlinks",function(e,c){var n="https://groups.google.com",a=n+"/"+c.wizPath,i=n+"/?web=",n=("0"!==c.account.authuser&&(i+="&authuser="+c.account.authuser),cjce.createElement("bm-ogb",{serviceIcon:"groups",pageLabel:cjBasics.lang.i18n("cj_i18n_00291","Home"),serviceLabel:cjBasics.lang.i18n("cj_i18n_00418","Groups"),bmApis:c,onNewTab:function(){c.openTab(i)},searchbox:{onSubmit:function(e){o("search?q="+encodeURIComponent(e))},submitInNewTab:!0}})),n=(e.appendChild(n),c.setOnPageDisplayHandler(n.cjceSearchboxEl.select),cjce.createElement("cjmd-container",{scrollable:!0,shadow:"thinOnScroll"})),s={"my-groups":{label:cjBasics.lang.i18n("cj_i18n_00388","My groups"),iconName:"__mdi:people",newTabUrl:!0},recent:{label:cjBasics.lang.i18n("cj_i18n_06607","Recent groups"),iconName:"__mdi:access_time",newTabUrl:!0},"starred-conversations":{label:cjBasics.lang.i18n("cj_i18n_06827","Starred conversations"),iconName:"__mdi:star_outline",newTabUrl:!0,divider:!0},search:{label:cjBasics.lang.i18n("cj_i18n_00085","Search"),iconName:"__mdi:search",newTabUrl:!0},"search-groups":{label:cjBasics.lang.i18n("cj_i18n_07057","Search groups"),iconName:"__mdi:search",newTabUrl:!0},"search-conversations":{label:cjBasics.lang.i18n("cj_i18n_07058","Search conversations"),iconName:"__mdi:search",newTabUrl:!0}};function o(e){c.openTab(a+(e||""))}function r(e,c){o("home"===e?"my-groups":e)}s=cjce.createElement("bm-navlist",{items:s,onClick:r,onNewTab:r});n.appendChild(s),e.appendChild(n)});