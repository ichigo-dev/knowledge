"use strict";cjce.registerTemplate("bm-p-fi",function(e,c){var a="https://fi.google.com/account",n=("0"!==c.account.authuser&&(a+="?authuser="+c.account.authuser),cjce.createElement("bm-ogb",{drawer:!0,serviceIcon:"fi",serviceLabel:cjBasics.lang.i18n("cj_i18n_06399","Google Fi"),pageLabel:cjBasics.lang.i18n("cj_i18n_00291","Home"),bmApis:c,onNewTab:function(){l("account")}})),i=(e.appendChild(n),cjce.createElement("cjmd-container",{scrollable:!0,shadow:"thinOnScroll"}));function l(e){c.openTab(a+"#"+e)}function t(e,a){l(a.newTabUrl)}var o=[{label:cjBasics.lang.i18n("cj_i18n_00481","Account"),newTabUrl:"account",iconName:"__mdi:person"},{label:cjBasics.lang.i18n("cj_i18n_01820","Your plan"),newTabUrl:"plan",iconName:"__mdi:signal_cellular_alt"},{label:cjBasics.lang.i18n("cj_i18n_00251","Billing"),newTabUrl:"billing",iconName:"__mdi:receipt"},{label:cjBasics.lang.i18n("cj_i18n_01821","Shop"),newTabUrl:"buydevices",iconName:"__mdi:store"},{label:cjBasics.lang.i18n("cj_i18n_01140","Support"),newTabUrl:"support",iconName:"__mdi:help_outline"},{header:!0,color:!0,label:cjBasics.lang.i18n("cj_i18n_00298","Settings")},{label:cjBasics.lang.i18n("cj_i18n_01822","Service address"),newTabUrl:"settings/emergency",iconName:"__mdi:place"},{label:cjBasics.lang.i18n("cj_i18n_00155","History"),newTabUrl:"callhistory",iconName:"__mdi:history"},{label:cjBasics.lang.i18n("cj_i18n_01484","Alerts"),newTabUrl:"settings/alerts",iconName:"__mdi:notifications"},{label:cjBasics.lang.i18n("cj_i18n_00901","Privacy"),newTabUrl:"settings/privacy",iconName:"__mdi:lock"}],s={home:{iconName:"__mdi:home",label:cjBasics.lang.i18n("cj_i18n_00291","Home"),newTabUrl:a+"#account"},relatedHeader:{header:!0,color:!0,label:cjBasics.lang.i18n("cj_i18n_01539","Related apps")},chat:{iconName:"__mdi:chat_bubble_outline",label:cjBasics.lang.i18n("cj_i18n_06694","Google Chat"),shortcutId:"559",external:!c.servicesDatabase.chat},voice:{iconName:"__mdi:google_voice",label:cjBasics.lang.i18n("cj_i18n_00443","Voice"),shortcutId:"154",external:!c.servicesDatabase.voice}},o=cjce.createElement("bm-navlist",{items:o,onClick:t,onNewTab:t}),o=(i.appendChild(o),e.appendChild(i),n.cjceDrawerEl),r=cjce.createElement("bm-navlist",{isSelector:!0,selectedId:"home",items:s,onChange:function(e){c.pageManager.navigate(e),r.cjceSelectedId="home"},onNewTab:function(e,a){a=a.newTabUrl||cjgShortcuts.getUrl(a.shortcutId,c.account);c.openTab(a)}});o.appendChild(r)});