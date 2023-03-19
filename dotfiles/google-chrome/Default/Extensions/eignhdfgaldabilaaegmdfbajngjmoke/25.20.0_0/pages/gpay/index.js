"use strict";cjce.registerTemplate("bm-p-gpay",function(e,n){var a=n.instanceParameters.productId||"gpay",i="https://pay.google.com/gp/w/u/"+n.account.authuser+"/home/",c="0"===n.account.authuser?"":"?authuser="+n.account.authuser,s="activity",l=i+s,t={gp_activity:{label:cjBasics.lang.i18n("cj_i18n_01515","Activity"),iconName:"__mdi:receipt",newTabUrl:!0},act_activity:{label:cjBasics.lang.i18n("cj_i18n_07191","Google Pay experience"),iconName:"__mdi:receipt",newTabUrl:"https://myactivity.google.com/"+n.wizPath+"product/gpay"},gp_paymentmethods:{iconName:"__mdi:payment",label:cjBasics.lang.i18n("cj_i18n_01517","Payment methods"),newTabUrl:!0},gp_subscriptionsandservices:{iconName:"__mdi:repeat",label:cjBasics.lang.i18n("cj_i18n_01513","Subscriptions & services"),newTabUrl:!0},gp_addressbook:{iconName:"__mdi:place",label:cjBasics.lang.i18n("cj_i18n_01518","Addresses"),newTabUrl:!0},gp_settings:{iconName:"__mdi:settings",label:cjBasics.lang.i18n("cj_i18n_00298","Settings"),newTabUrl:!0},hd_advanced:{header:!0,color:!0,label:cjBasics.lang.i18n("cj_i18n_01588","Advanced")},hd_account:{header:!0,color:!0,label:cjBasics.lang.i18n("cj_i18n_07187","Transactions via Search, Maps and Assistant")},hd_gmail:{header:!0,color:!0,label:cjBasics.lang.i18n("cj_i18n_07189","Transactions in Gmail")},hd_setup:{header:!0,color:!0,label:cjBasics.lang.i18n("cj_i18n_07190","Setup")},ga_reservations:{iconName:"__mdi:local_activity",label:cjBasics.lang.i18n("cj_i18n_01087","Reservations"),newTabUrl:"https://myaccount.google.com/"+n.wizPath+"reservations"},gm_reservations:{iconName:"__mdi:local_activity",label:cjBasics.lang.i18n("cj_i18n_01087","Reservations"),newTabUrl:"https://mail.google.com/mail/u/"+n.account.authuser+"/#search/category%3Areservations"},ga_purchases:{iconName:"__mdi:shopping_cart",label:cjBasics.lang.i18n("cj_i18n_00310","Purchases"),newTabUrl:"https://myaccount.google.com/"+n.wizPath+"purchases"},gm_purchases:{iconName:"__mdi:shopping_cart",label:cjBasics.lang.i18n("cj_i18n_00310","Purchases"),newTabUrl:"https://mail.google.com/mail/u/"+n.account.authuser+"/#search/category%3Apurchases"},gp_usermanagement:{iconName:"__mdi:people",label:cjBasics.lang.i18n("cj_i18n_01519","Manage users"),newTabUrl:!0},gp_taxdata:{iconName:"__mdi:account_balance",label:cjBasics.lang.i18n("cj_i18n_01520","Add tax info"),newTabUrl:!0},gp_documentcenter:{iconName:"__mdi:insert_drive_file",label:cjBasics.lang.i18n("cj_i18n_01521","Documents"),newTabUrl:!0},gp_customerorders:{iconName:"__mdi:shopping_cart",label:cjBasics.lang.i18n("cj_i18n_01522","Customer orders"),newTabUrl:!0},gp_salestax:{iconName:"__mdi:account_balance",label:cjBasics.lang.i18n("cj_i18n_01523","Sales tax"),newTabUrl:!0},hd_resources:{header:!0,color:!0,label:cjBasics.lang.i18n("cj_i18n_06648","Resources")},about:{iconName:"__mdi:info",label:cjBasics.lang.i18n("cj_i18n_00838","About"),newTabUrl:"https://pay.google.com/about/"},business:{iconName:"__mdi:business",label:cjBasics.lang.i18n("cj_i18n_01071","G Pay for Business"),newTabUrl:"https://pay.google.com/about/business/"},developers:{iconName:"__mdi:code",label:cjBasics.lang.i18n("cj_i18n_06904","G Pay for Developers"),newTabUrl:"https://developers.google.com/pay/"+c},cardIssuer:{iconName:"__mdi:attach_money",label:cjBasics.lang.i18n("cj_i18n_06425","Card Issuer"),newTabUrl:"https://developers.google.com/pay/issuers"+c},help:{iconName:"__mdi:help",label:cjBasics.lang.i18n("cj_i18n_00438","Help"),newTabUrl:"https://support.google.com/pay"+c},merchantCenter:{iconName:"__mdi:shopping_cart",label:cjBasics.lang.i18n("cj_i18n_00493","Merchant Center"),newTabUrl:"https://pay.google.com/gp/m/issuer/list"+c}};function o(e,a){var c=a.newTabUrl;!0===c&&(a=a&&a.id||e||s,c=i+a.replace("gp_","")),n.openTab(c)}function r(e){if(e){var a=e.toLowerCase(),c=_.querySelectorAll(".cjmd-item");for(l=0;l<c.length;l++){var n=c[l];n.hidden=-1===n.textContent.toLowerCase().indexOf(a)}var i=_.querySelectorAll(".cjmd-subheader");for(l=0;l<i.length;l++)i[l].hidden=!0}else for(var s=_.querySelectorAll(".cjmd-item[hidden],.cjmd-subheader[hidden]"),l=0;l<s.length;l++)s[l].hidden=!1}var c=cjce.createElement("bm-ogb",{serviceIcon:"wallet"===a?"wallet":"pay",serviceLabel:"wallet"===a?cjBasics.lang.i18n("cj_i18n_00449","Wallet"):cjBasics.lang.i18n("cj_i18n_01524","Pay"),pageLabel:cjBasics.lang.i18n("cj_i18n_01167","Links"),bmApis:n,searchbox:{color:!0,onInput:r,onClear:r,onSubmit:function(){_.querySelector(".cjmd-item:not([hidden])").click()}},onNewTab:function(){n.openTab(l)}}),_=(e.appendChild(c),n.setOnPageDisplayHandler(c.cjceSearchboxEl.select),cjce.createElement("cjmd-container",{scrollable:!0,shadow:"thinOnScroll"})),c=(e.appendChild(_),"wallet"===a?["gp_activity","gp_subscriptionsandservices","hd_account","ga_reservations","ga_purchases","hd_gmail","gm_reservations","gm_purchases","hd_setup","act_activity","gp_paymentmethods","gp_addressbook","gp_settings"]:["gp_activity","gp_paymentmethods","gp_subscriptionsandservices","gp_addressbook","gp_settings","hd_advanced","gp_usermanagement","gp_customerorders","gp_documentcenter","gp_salestax","hd_resources","about","business","developers","cardIssuer","merchantCenter"]),e=cjce.createElement("bm-navlist",{compact:"wallet"!==a,items:c.map(function(e){var a=t[e];return a.id=e,a}),onClick:o,onNewTab:o});_.appendChild(e)});