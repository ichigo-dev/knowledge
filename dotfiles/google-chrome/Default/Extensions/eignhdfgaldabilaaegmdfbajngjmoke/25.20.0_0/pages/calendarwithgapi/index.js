"use strict";!function(){var Ee=window,Se=document,Ne=browserStorage.sync.getItem("bm_pref__calendar__hidereminders");cjce.registerTemplate("bm-p-calendarwithgapi",function(a,r){var i,e,c,o,R,l,s,d,m,u,M,O,U,p,h,t,q,g,_,n,f=["https://www.googleapis.com/auth/calendar"],F=["https://www.googleapis.com/auth/reminders"],H=Ee.atob("QUl6YVN5Q0FmRFBsWlJMam1nSENKTnVpTVQ2UkNHVFBVVC1CU05F"),b="https://content.googleapis.com/calendar/v3/",j=cjBasics.lang.i18n("cj_i18n_00302","Calendar"),V="https://keep.google.com/"+r.wizPath,v="https://calendar.google.com",Y=v+"/calendar/u/"+r.account.authuser+"/",C=Y+"r",z=(cjBasics.uniqueId.generate(),!1),J=!1,w=cjBasics.datetime.fullMonthNames,K=cjBasics.datetime.shortWeekdayNames,y=new Date,Q=y.getFullYear(),T=y.getMonth(),G=y.getDate(),W=y.getDay(),y=y.getTime(),Z=Q+"_"+T.toString(16),E=26784e5,S=y-E,N=y+E,$=new Date(S).toISOString(),X=new Date(N).toISOString(),k=!1,ee=r.debugMode&&!1,te=[16,12,22,1,11,3,13,18,15,7,2],ne=["#16a765","#42d692","#4986e7","#7bd148","#92e1c0","#9a9cff","#9fc6e7","#9fe1e7","#a47ae2","#ac725e","#b3dc6c","#b99aff","#c2c2c2","#cabdbf","#cca6ac","#cd74e6","#d06b64","#f691b2","#f83a22","#fa573c","#fad165","#fbe983","#ff7537","#ffad46"],B=["#795548","#e67c73","#d50000","#f4511e","#ef6c00","#f09300","#00897b","#0b8043","#7cb342","#c0ca33","#e4c441","#f6bf26","#33b679","#039be5","#4285f4","#3f51b5","#7986cb","#b39ddb","#616161","#a79b8e","#ad1457","#d81b60","#8e24aa","#9e69af"];function ae(t,n){var a,c,e=Se.createDocumentFragment(),i=cjce.createElement("ccbm-iconbutton",{iconName:"__mdi:edit",label:cjBasics.lang.i18n("cj_i18n_05629","Edit"),onClick:function(e){ge(t||h,"edit")}}),i=(e.appendChild(i),cjce.createElement("ccbm-iconbutton",{iconName:"__mdi:delete",label:cjBasics.lang.i18n("cj_i18n_01563","Delete"),onClick:function(e){fe(t||h).then(function(){n&&n.remove(),ie(p=!1),O.cjceSendFrameCommand({method:"calendarirenderShowEventClose"},v)})}})),i=(e.appendChild(i),a=t,c=n,cjce.createElement("ccbm-iconbuttonmenu",{compact:!0,label:cjBasics.lang.i18n("cj_i18n_06294","More actions"),items:{duplicate:{iconName:"__mdi:content_copy",label:cjBasics.lang.i18n("cj_i18n_06293","Duplicate")},print:{iconName:"__mdi:print",label:cjBasics.lang.i18n("cj_i18n_02543","Print")}},onClick:function(e){"delete"===e?fe(a||h).then(function(){c&&c.remove()}):ge(a||h,e)}}));return e.appendChild(i),e}function I(e){var t,n;return"not_authorized"!==e.cjg_error||J||(J=!0,u&&(u.remove(),u.textContent="",u=null),cjgApis.cache.remove(r.account,"bm_cache_v33__calendar__eventlist"),t=cjce.createElement("cjmd-container",{shadow:"thin"}),n=cjBasics.lang.i18n("cj_i18n_00291","Home"),l.cjceSetPageLabel(n),s.hidden=!1,m.hidden=!0,d.hidden=!0,s.classList.remove("bm-p-calendarwithgapi-searchbox"),n=cjce.createElement("bm-fulldialog",{inline:!0,lockup:{label:j},iconName:"calendar",message:cjBasics.lang.i18n("cj_i18n_02176","Before you can use this page, you need to give access to load your __product_name__ data").replace("__product_name__",j),actionLabel:cjBasics.lang.i18n("cj_i18n_01916","Continue with Google"),actionG:!0,action:function(){cjgApis.auth.requestPermissions(r.account,e.cjg_missing_scopes)}}),t.appendChild(n),a.appendChild(t)),Promise.reject(e)}function L(e,t){t=t||{},t.prettyPrint="false",e=cjBasics.urlParams.attach(b+e,t);return cjgApis.request(e,{fetchAs:"json"},{account:r.account,requiredScopes:f}).catch(I)}function ce(e){R=new Intl.DateTimeFormat(cjBasics.lang.current,e)}function D(t){t={timeStyle:"short",hour:"numeric",minute:"numeric",hour12:!t};try{ce(t)}catch(e){delete t.timeStyle,ce(t)}}function ie(e){!1===e&&p&&(p=!1,l.cjceSetLoading(!0),setTimeout(P,1e3)),e!==k&&((k=e)?setTimeout(function(){k&&M.classList.remove("bm-p-calendarwithgapi-inlinerender--hidden")}):M.classList.add("bm-p-calendarwithgapi-inlinerender--hidden"))}function re(){return L("users/me/calendarList",{maxResults:"250",fields:"items(backgroundColor,foregroundColor,id,selected,summary,colorId)"}).then(function(e){return e.items||[]},function(){return[]})}function oe(e,t){var n,a;return e.allday!==t.allday?e.allday?-1:1:e.start!==t.start?e.start-t.start:e.isReminder!==t.isReminder?e.isReminder?-1:1:(n=(e.title||"").toLowerCase())!==(a=(t.title||"").toLowerCase())?n<a?-1:1:e.eventId<t.eventId?-1:1}function A(e){return e?e.charAt(0).toUpperCase()+e.slice(1):cjBasics.lang.i18n("cj_i18n_01737","(No title)")}function le(){return cjgApis.cache.getItem(r.account,"bm_cache_v33__calendar__reminders").then(function(e){return Array.isArray(e)?e:[]})}function se(e){return cjgApis.cache.setItem(r.account,"bm_cache_v33__calendar__reminders",e),e}function de(e){return R.format(e).replace(" AM","am").replace(" PM","pm")}function me(){return e=N,t=S,n=cjBasics.urlParams.attach("https://reminders-pa.googleapis.com/v1internalOP/reminders/list",{fields:"task(dueDate,title,extensions,externalApplicationLink)",prettyPrint:"false"}),cjgApis.request(n,{method:"POST",fetchAs:"json",headers:{"Content-Type":"application/json"},body:JSON.stringify({includeDeleted:!1,includeArchived:!0,dueBeforeMs:e,dueAfterMs:t})},{pa:!0,key:H,account:r.account,requiredScopes:F}).then(function(e){return e.task.filter(function(e){return e&&e.dueDate&&e.dueDate.year})}).then(se,le).then(function(e){return e.map(function(e){a=(a=e).dueDate,(c=new Date).setFullYear(a.year),c.setMonth(a.month-1),c.setDate(a.day),(a=a.time)&&(c.setHours(a.hour),c.setMinutes(a.minute),c.setSeconds(a.second));var t,n,a=c,c=a.getTime();return{isReminder:!0,title:A(e.title),keepData:(n=(n=(n=(t=e).extensions)&&n.keepExtension&&n.keepExtension)&&(n.serverNoteId||n.clientNoteId))?{url:V+"#NOTE/"+n,noteId:n}:(t=(n=t.externalApplicationLink)&&"keepReminder"===n.application&&n.id)?{url:V+"?reminder="+t,reminderId:t}:void 0,start:c,allday:e.dueDate.allDay,timeString:de(a)}})}).catch(function(){return[]});var e,t,n}function ue(){var e,t=u.querySelector(".bm-p-calendarwithgapi-datebadge--selected");t&&(e=u.scrollTop,t=t.offsetTop-128,Math.abs(e-t)<50||(u.scrollTop=t))}function x(){u&&(u.classList.remove("bm-p-calendarwithgapi-container--smoothscroll"),ue(),u.classList.add("bm-p-calendarwithgapi-container--smoothscroll"))}function pe(e){e=cjBasics.urlParams.attach(C+"/search",{q:e});r.openTab(e)}function he(e){return Ee.btoa(e.eventId+" "+e.calendarId)}function ge(e,t){var e=he(e),n=Y,a={},t=("edit"===t?n+="r/eventedit/"+e:"duplicate"===t?n+="r/eventedit/duplicate/"+e:a="print"===t?(n+="printevent",{sf:"true",eid:e}):(n+="r/",{eid:e}),cjBasics.urlParams.attach(n,a));r.openTab(t)}function _e(e){var t=C+"/eventedit",t=cjBasics.urlParams.attach(t,{vcon:e?"meet":null,details:"Created with https://apps.jeurissen.co/black-menu-for-google"});r.openTab(t)}function fe(e){var t=encodeURIComponent(e.calendarId),e=encodeURIComponent(e.eventId);return cjgApis.request(b+"calendars/"+t+"/events/"+e,{method:"DELETE"},{account:r.account,requiredScopes:f})}function be(){var t=[],n=Promise.all([c,e]).then(function(e){return o?me(e[1]):[]}).then(function(e){t=t.concat(e)},function(){});return Promise.all([i,e]).then(function(e){e=e[0].map(function(e){return!0!==e.selected?Promise.resolve():(s=e,L("calendars/"+encodeURIComponent(s.id)+"/events",{timeMin:$,timeMax:X,maxResults:"2500",singleEvents:"true",fields:"items(id,end/dateTime,endTimeUnspecified,start(date,dateTime),summary,colorId)"}).then(function(e){return e.items.map(function(e){var t,n=new Date(e.start.dateTime||e.start.date+"T12:00:00Z"),a=n.getTime(),c="date"in e.start,i=null,r=(c||(i=de(n),!e.endTimeUnspecified&&e.end&&e.end.dateTime&&(r=(n=new Date(e.end.dateTime)).getTime(),i+=" - "+de(new Date(e.end.dateTime)),864e5<r-a)&&(i+=" ("+cjBasics.datetime.printDateString(n)+")")),e&&e.colorId),n=s.colorId,o=s.backgroundColor,l=o&&-1===ne.indexOf(o),r=(!l&&n&&(o=B[Number(n)-1]),r&&(t=o,o=B[te[Number(r)-1]]),!r&&(!n||l)&&"#000000"===s.foregroundColor);return{calendarId:s.id,eventId:e.id,isReminder:!1,backgroundColor:o,sidestampColor:t,lightMode:r,title:A(e.summary),start:a,allday:c,timeString:i}})},I).catch(function(){return[]}).then(function(e){e&&(t=t.concat(e))}));var s});return e.push(n),Promise.all(e).then(function(){return t.sort(function(e,t){return e.start-t.start})})})}function je(n){var a,e=Se.createElement("div"),t=(e.classList.add("bm-p-calendarwithgapi-eventlist__item"),A(n.title)),c=!n.allday,i=(n.isReminder?(e.classList.add("bm-p-calendarwithgapi-eventlist__item--reminder"),c&&(t+=", "+n.timeString),(i=cjce.createElement("cjmd-icon",{theme:"solid",color:"#fff",name:"__mdi:reminders_alt",size:16})).classList.add("bm-p-calendarwithgapi-eventlist__remindericon"),e.appendChild(i)):c&&(t+="\n"+n.timeString),n.backgroundColor&&(e.style.backgroundColor=n.backgroundColor),n.sidestampColor&&(e.style.borderLeftColor=n.sidestampColor),n.lightMode&&e.classList.add("bm-p-calendarwithgapi-eventlist__item--light"),Se.createElement("span"));return i.classList.add("bm-p-calendarwithgapi-eventlist__label"),i.textContent=t,e.appendChild(i),n.isReminder||((a=Se.createElement("div")).className="bm-p-calendarwithgapi-eventlist__tools",e.appendChild(a),c=ae(n,e),a.appendChild(c)),n.keepData&&((a=Se.createElement("div")).className="bm-p-calendarwithgapi-eventlist__tools",e.appendChild(a),(t=cjce.createElement("ccbm-iconbutton",{iconName:"__mdi:drive_keep",label:cjBasics.lang.i18n("cj_i18n_06300","View related note in Keep")})).classList.add("bm-p-calendarwithgapi-eventlist__keepbutton"),a.appendChild(t)),e.addEventListener("click",function(e){var t=e.target;t===a||a.contains(t)||(n.isReminder?n.keepData&&(t=cjBasics.events.hasMeta(e)||e.shiftKey,e=n.keepData,t?r.openTab(e.url):r.pageManager.navigate("keep",{noteData:e})):ee?(p=!1,U=he(h=n),O.cjceSendFrameCommand({method:"calendarirenderShowEventActivate",value:U},v),setTimeout(function(){ie(!((void 0).hidden=!1))})):ge(n))}),e}function ve(e,t){return{fullName:w[t]+" "+e,month:t,dates:[]}}function Ce(e,t,n,a,c){return{weekdayName:K[a],dateNumber:n,events:[],isToday:c||!1}}function we(e){var a,t,o;u&&(0===e.length?(u.textContent="",t=cjce.createElement("cjmd-emptystate",{iconName:"__mdi:event",label:cjBasics.lang.i18n("cj_i18n_00265","No events found"),subLabel:cjBasics.lang.i18n("cj_i18n_00266","To add a new event, click the red button in the bottom right corner")}),u.appendChild(t)):(a=Se.createDocumentFragment(),t=e,o={},e=ve(Q,T),(o[Z]=e).dates[G]=Ce(0,0,G,W,!0),t.forEach(function(e){var t=new Date(e.start),n=t.getFullYear(),a=t.getMonth(),c=t.getDate(),i=n+"_"+a.toString(16),r=o[i],n=(r||(r=ve(n,a),o[i]=r),r.dates[c]);n||(n=Ce(0,0,c,t.getDay(),!1),r.dates[c]=n),n.events.push(e)}),Object.keys(o).sort().map(function(e){return o[e]}).forEach(function(e,t){var n=Se.createElement("div");n.className="bm-p-calendarwithgapi-monthlist",n.dataset.month=e.month,a.appendChild(n),0<t&&((t=Se.createElement("div")).className="bm-p-calendarwithgapi-monthlist__header",t.textContent=e.fullName,n.appendChild(t)),e.dates.forEach(function(e){e=function(e){var t=Se.createElement("div"),n=(t.className="bm-p-calendarwithgapi-monthlist__item",Se.createElement("div")),a=(n.className="bm-p-calendarwithgapi-datebadge",t.appendChild(n),e.isToday&&n.classList.add("bm-p-calendarwithgapi-datebadge--selected"),Se.createElement("div"));a.className="bm-p-calendarwithgapi-datebadge__date",a.textContent=e.dateNumber,n.appendChild(a);(a=Se.createElement("div")).className="bm-p-calendarwithgapi-datebadge__weekday",a.textContent=e.weekdayName,n.appendChild(a);var c=Se.createElement("div");return c.className="bm-p-calendarwithgapi-eventlist",t.appendChild(c),0===e.events.length?((n=Se.createElement("div")).className="bm-p-calendarwithgapi-eventlist__noresults",n.textContent=e.isToday?cjBasics.lang.i18n("cj_i18n_00267","No events today"):cjBasics.lang.i18n("cj_i18n_06337","No events on this date"),c.appendChild(n)):e.events.sort(oe).forEach(function(e){e=je(e);c.appendChild(e)}),t}(e);n.appendChild(e)})}),u.textContent="",u.appendChild(a),setTimeout(function(){0===u.scrollTop?u.scrollTop=50:u.scrollTop+u.offsetHeight===u.scrollHeight&&(u.scrollTop-=50)},10)))}function P(){return l.cjceSetLoading(!0),be().then(function(e){z=!0,cjgApis.cache.setItem(r.account,"bm_cache_v33__calendar__eventlist",e),we(e),l.cjceSetLoading(!1)})}function ye(a,e){var t,n,c=cjce.createElement("bm-navitem",{icon:{name:"__mdi:check_box",theme:"solid"},color:(n=a).isRemindersCalendar?"#3f51b5":(t=n.backgroundColor)&&-1===ne.indexOf(t)?t:n.colorId&&B[Number(n.colorId)-1]||t||"#1a73e8",colorIcon:!0,label:a.summary,divider:e});return c.classList.add("bm-p-calendarwithgapi-draweritem"),c.addEventListener("click",function(e){var t,n=c.classList.toggle("bm-p-calendarwithgapi-draweritem--inactive");a.isRemindersCalendar?(o=!n,browserStorage.sync.setItem("bm_pref__calendar__hidereminders",n).then(function(){P().then(x)})):(t=a.id,n=!n,t=b+"users/me/calendarList/"+encodeURIComponent(t)+"?fields=",cjgApis.request(t,{method:"PATCH",body:JSON.stringify({selected:n}),headers:{"Content-Type":"application/json"}},{account:r.account,requiredScopes:f}).catch(I).then(function(){i=re(),P().then(x)}))}),a.selected||c.classList.add("bm-p-calendarwithgapi-draweritem--inactive"),a.isRemindersCalendar||((n=cjce.createElement("ccbm-iconbutton",{iconName:"__mdi:settings",label:cjBasics.lang.i18n("cj_i18n_01254","Settings and sharing"),onClick:function(e){var t;r.openTab(C+"/settings/calendar/"+(t=a.id,Ee.btoa(t).replace(/\+/g,"-").replace(/\//g,"_").replace(/=+$/,"")))}})).classList.add("bm-p-calendarwithgapi-draweritem__settings"),c.appendChild(n)),c}function Te(e){l.cjceSetNavigatorCollapseState(e),l.cjceSetBackState(e),s.hidden=!e,d.hidden=e,m.hidden=e}c=Ne.then(function(e){o=!e}),i=re(),t=!1,q=L("users/me/settings/format24HourTime",{fields:"value"}).then(function(e){t=!0;e="object"!=typeof e||"false"!==e.value&&!1!==e.value;return cjgApis.cache.setItem(r.account,"bm_cache_v33__calendar__24hours",e),D(e),e},function(){return!0}),e=cjgApis.cache.getItem(r.account,"bm_cache_v33__calendar__24hours").then(function(e){return t||"boolean"!=typeof e?q:(D(e),e)}),l=cjce.createElement("bm-ogb",{loading:!0,drawer:{keepOpenOnClick:!0},serviceIcon:"calendar",serviceLabel:j,pageLabel:w[T],searchbox:{collapsable:!0,onSubmit:pe,submitInNewTab:!0},bmApis:r,onNewTab:function(){r.openTab(C)},onBack:function(){Te(!1)}}),a.appendChild(l),y=l.cjceDrawerEl,(s=l.cjceSearchboxEl).classList.add("bm-p-calendarwithgapi-searchbox"),s.hidden=!0,r.setOnPageDisplayHandler(s.select),m=cjce.createElement("ccbm-button",{outline:!0,label:cjBasics.lang.i18n("cj_i18n_01253","Today"),onClick:ue}),l.cjceAppendChild(m),d=cjce.createElement("ccbm-iconbutton",{label:cjBasics.lang.i18n("cj_i18n_00085","Search"),iconName:"__mdi:search",onClick:function(){Te(!0),s.select()}}),l.cjceAppendChild(d),(g=cjce.createElement("cjmd-container",{scrollable:!0,fabPadding:!0,solid:!0,shadow:"thinOnScroll",onScrollTop:function(){S-=E,$=new Date(S).toISOString(),P()},onScrollBottom:function(){N+=E,X=new Date(N).toISOString(),P()}})).addEventListener("scroll",function(){for(var e=g.querySelectorAll(".bm-p-calendarwithgapi-monthlist"),t=e[0].dataset.month||T,n=1;n<e.length&&!(g.scrollTop<e[n].offsetTop-68);n++)t=e[n].dataset.month;l.cjceSetPageLabel(w[t])}),u=g,a.appendChild(u),D(),P().then(x),cjgApis.cache.getItem(r.account,"bm_cache_v33__calendar__eventlist").then(function(e){e&&0<e.length&&!z&&(we(e),x())}),_=cjce.createElement("cjmd-list"),y.appendChild(_),Promise.all([i,c]).then(function(n){n[0].concat({isRemindersCalendar:!0,selected:o,summary:cjBasics.lang.i18n("cj_i18n_00081","Reminders")}).sort(function(e,t){return e.summary<t.summary?-1:1}).forEach(function(e,t){e=ye(e,t===n[0].length);_.appendChild(e)}),_.lastChild.classList.add("cjmd-item--divider")}),n={trash:{iconName:"__mdi:delete",label:cjBasics.lang.i18n("cj_i18n_00005","Bin"),newTabUrl:C+"/trash",external:!0},settings:{iconName:"__mdi:settings",label:cjBasics.lang.i18n("cj_i18n_00298","Settings"),newTabUrl:C+"/settings",external:!0}},n=cjce.createElement("bm-navlist",{items:n,onNewTab:function(e,t){t=t.newTabUrl||cjgShortcuts.getUrl(t.shortcutId,r.account);r.openTab(t)}}),y.appendChild(n),n=[cjce.createElement("gmd-fab",{label:cjBasics.lang.i18n("cj_i18n_00268","New event"),onClick:function(){_e()},onMetaClick:_e}),cjce.createElement("ccbm-fab",{mini:!0,color:"#00897b",iconName:"__mdi:video_call",label:cjBasics.lang.i18n("cj_i18n_06947","New event with meeting"),onClick:function(){_e(!0)}})],n=cjce.createElement("ccbm-fabgroup",{items:n}),a.appendChild(n)})}();