"use strict";!function(){var ie=window,ae=document;cjce.registerTemplate("bm-p-drive",function(o,l){var r,t,s,e,i,d,m,p,_,u,g,h,n,v,f,b="my-drive",j=b,c=b,y=!1,a=(Math.round(ae.documentElement.offsetHeight*ae.documentElement.offsetWidth/7500),"https://drive.google.com"),T=a+"/drive/"+l.wizPath;function w(){var e=T+("folder"===b||"shareddrive"===b?"folders/"+g.id:"search"===b?"search?q="+encodeURIComponent(m.value):b);l.openTab(e)}var B,A=["trash","starred","my-drive","folder"],M=ie.atob("QUl6YVN5RFA3MkZMTVpCNng0ejZDVnlCOG8wQUthWC0yRDdueGMw"),C="https://content.googleapis.com",S=C+"/drive/v3/",x=["https://www.googleapis.com/auth/drive"],I=["https://www.googleapis.com/auth/drive.apps.readonly"],U=!0,O=cjgApis.auth.requireToken(l.account,x).catch(function(e){return U=!1,e}),D={},F=["196802322321","230754619982","335078253515","619683526622","629453589428","796396377186","889782162350","897606708560","952342923413","371237729773","1083656409722"],R={name:cjBasics.lang.i18n("cj_i18n_01990","Name"),lastModified:cjBasics.lang.i18n("cj_i18n_00093","Last modified"),lastModifiedByMe:cjBasics.lang.i18n("cj_i18n_00094","Last modified by me"),lastOpenedByMe:cjBasics.lang.i18n("cj_i18n_00095","Last opened by me")},J={name:"name",lastModified:"modifiedTime desc",lastModifiedByMe:"modifiedByMeTime desc",lastOpenedByMe:"viewedByMeTime desc"},G={archive:"(mimeType = 'application/bzip2' or mimeType = 'application/gzip' or mimeType = 'application/gzip-compressed' or mimeType = 'application/gzipped' or mimeType = 'application/rar' or mimeType = 'application/tar' or mimeType = 'application/x-bzip' or mimeType = 'application/x-bzip-compressed-tar' or mimeType = 'application/x-bzip2' or mimeType = 'application/x-gtar' or mimeType = 'application/x-gtar-compressed' or mimeType = 'application/x-gunzip' or mimeType = 'application/x-gzip' or mimeType = 'application/x-gzip-compressed' or mimeType = 'application/x-rar' or mimeType = 'application/x-tar' or mimeType = 'application/x-tgz' or mimeType = 'application/x-zip' or mimeType = 'application/x-zip-compressed' or mimeType = 'application/zip' or mimeType = 'gzip/document' or mimeType = 'multipart/x-gzip' or mimeType = 'multipart/x-rar' or mimeType = 'multipart/x-tar' or mimeType = 'multipart/x-zip' or mimeType = 'multipart/zip')",audio:"(mimeType = 'audio/3gp' or mimeType = 'audio/midi' or mimeType = 'audio/mp4a-latm' or mimeType = 'audio/mpeg' or mimeType = 'audio/mpeg3' or mimeType = 'audio/ogg' or mimeType = 'audio/x-ms-wma' or mimeType = 'audio/x-wav' or mimeType = 'audio/wav')",document:"(mimeType = 'application/msword' or mimeType = 'application/vnd.google-apps.document' or mimeType = 'application/vnd.google-apps.kix' or mimeType = 'application/vnd.ms-word' or mimeType = 'application/vnd.oasis.opendocument.text' or mimeType = 'application/vnd.openxmlformats-officedocument.wordprocessingml.document' or mimeType = 'text/plain')",drawing:"(mimeType = 'application/vnd.oasis.opendocument.graphics' or mimeType = 'image/vnd.adobe.photoshop' or mimeType = 'image/x-photoshop' or mimeType = 'application/illustrator' or mimeType = 'application/vnd.google-apps.drawing')",folder:"(mimeType = 'application/vnd.google-apps.folder')",form:"(mimeType = 'application/vnd.google-apps.form' or mimeType = 'application/vnd.google-apps.freebird')",image:"(mimeType = 'image/jpeg' or mimeType = 'image/png' or mimeType = 'image/gif' or mimeType = 'image/tiff' or mimeType = 'image/x-ms-bmp' or mimeType = 'image/svg+xml' or mimeType = 'image/vnd.microsoft.icon')",pdf:"(mimeType = 'application/pdf')",presentation:"(mimeType = 'application/mspowerpoint' or mimeType = 'application/vnd.ms-powerpoint' or mimeType = 'application/vnd.oasis.opendocument.presentation' or mimeType = 'application/vnd.openxmlformats-officedocument.presentationml.presentation' or mimeType = 'application/vnd.google-apps.presentation' or mimeType = 'application/vnd.google-apps.punch')",spreadsheet:"(mimeType = 'application/msexcel' or mimeType = 'application/vnd.google-apps.form' or mimeType = 'application/vnd.google-apps.freebird' or mimeType = 'application/vnd.google-apps.ritz' or mimeType = 'application/vnd.google-apps.spreadsheet' or mimeType = 'application/vnd.ms-excel' or mimeType = 'application/vnd.oasis.opendocument.spreadsheet' or mimeType = 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet')",video:"(mimeType = 'video/3gpp' or mimeType = 'video/avi' or mimeType = 'video/dv' or mimeType = 'video/flv' or mimeType = 'video/mp2p' or mimeType = 'video/mp2t' or mimeType = 'video/mp4' or mimeType = 'video/mpeg' or mimeType = 'video/mpv' or mimeType = 'video/ogg' or mimeType = 'video/quicktime' or mimeType = 'video/x-flv' or mimeType = 'video/x-msvideo' or mimeType = 'video/x-ms-wmv' or mimeType = 'video/x-m4v' or mimeType = 'video/x-ms-asf' or mimeType = 'video/x-matroska' or mimeType = 'video/x-dv' or mimeType = 'video/webm')"},E={document:{fabLabel:cjBasics.lang.i18n("cj_i18n_00607","New document"),label:cjBasics.lang.i18n("cj_i18n_00391","Docs"),newTabUrl:cjgShortcuts.getUrl("299",l.account),icon:{size:32,name:"__mdi:drive_document",color:"#4285f4"}},spreadsheet:{fabLabel:cjBasics.lang.i18n("cj_i18n_00608","New spreadsheet"),label:cjBasics.lang.i18n("cj_i18n_00393","Sheets"),newTabUrl:cjgShortcuts.getUrl("300",l.account),icon:{size:32,name:"__mdi:drive_spreadsheet",color:"#34a853"}},presentation:{fabLabel:cjBasics.lang.i18n("cj_i18n_00609","New presentation"),label:cjBasics.lang.i18n("cj_i18n_00407","Slides"),newTabUrl:cjgShortcuts.getUrl("301",l.account),icon:{size:32,name:"__mdi:drive_presentation",color:"#fbbc05"}},upload:{fabLabel:cjBasics.lang.i18n("cj_i18n_01933","Upload file"),label:cjBasics.lang.i18n("cj_i18n_01934","File upload"),onClick:function(){e.click()},newTabUrl:"__bm_drive_upload",icon:{size:32,name:"__mdi:file_upload"}},folder:{fabLabel:cjBasics.lang.i18n("cj_i18n_01935","New folder"),label:cjBasics.lang.i18n("cj_i18n_01794","Folder"),newTabUrl:"__bm_drive_folder",onClick:function(){var a=cjce.createElement("ccbm-textfield",{label:cjBasics.lang.i18n("cj_i18n_01937","Folder title")}),n=cjBasics.lang.i18n("cj_i18n_01939","Untitled folder"),e=(a.cjceValue=n,cjce.createElement("cjmd-dialog",{darkMode:l.darkMode,title:cjBasics.lang.i18n("cj_i18n_01936","Create new folder"),message:cjBasics.lang.i18n("cj_i18n_01938","Please enter a folder title"),confirmLabel:cjBasics.lang.i18n("cj_i18n_00626","Create"),cancelLabel:cjBasics.lang.i18n("cj_i18n_01659","Cancel"),additionalFragment:a,onConfirm:function(){var e=a.cjceValue||n,i=(e={name:e,mimeType:"application/vnd.google-apps.folder"},"folder"!==b&&"shareddrive"!==b||(e.parents=[g.id]),C+"/drive/v3/files");cjgApis.request(i,{method:"POST",body:JSON.stringify(e),headers:{"Content-Type":"application/json; charset=UTF-8"}},{account:l.account,requiredScopes:x}).then(L)}}));ae.body.appendChild(e),setTimeout(function(){e.showModal(),e=null,a.cjceSelect()})},icon:{size:32,name:"__mdi:create_new_folder"}},form:{fabLabel:cjBasics.lang.i18n("cj_i18n_00631","New form"),label:cjBasics.lang.i18n("cj_i18n_00408","Forms"),newTabUrl:cjgShortcuts.getUrl("329",l.account),icon:{size:32,name:"__mdi:drive_form",color:"#673ab7"}},drawing:{label:cjBasics.lang.i18n("cj_i18n_00409","Drawings"),newTabUrl:cjgShortcuts.getUrl("47",l.account),icon:{size:32,name:"__mdi:drive_drawing",color:"#ea4335"}},atari:{label:cjBasics.lang.i18n("cj_i18n_00112","Sites"),newTabUrl:cjgShortcuts.getUrl("406",l.account),icon:{size:32,name:"__mdi:atari_site",color:"#3e50b4"}},map:{label:cjBasics.lang.i18n("cj_i18n_00410","My Maps"),newTabUrl:"https://www.google.com/maps/d/u/"+l.account.authuser+"/drive?state=%7B%22action%22%3A%22create%22%7D",icon:{size:32,name:"__mdi:file_map",color:"#ea4335"}},script:{label:cjBasics.lang.i18n("cj_i18n_00447","Apps Script"),newTabUrl:cjgShortcuts.getUrl("730",l.account),icon:{size:32,name:"__mdi:drive_script",color:"#1a73e8"}},jam:{label:cjBasics.lang.i18n("cj_i18n_00813","Jamboard"),newTabUrl:"https://jamboard.google.com/create?authuser="+l.account.authuser,icon:{size:32,name:"__mdi:cj_jam_drive",color:"#fb8c00"}}},V=["document","spreadsheet","presentation","form","drawing","atari","script","map","jam"].map(function(e){return E[e]}),H={pdf:{icon:{name:"__mdi:drive_pdf",color:"#ea4335"},label:cjBasics.lang.i18n("cj_i18n_00137","PDFs")},document:{icon:{name:"__mdi:drive_document",color:"#4285f4"},label:cjBasics.lang.i18n("cj_i18n_00138","Text documents")},spreadsheet:{icon:{name:"__mdi:drive_spreadsheet",color:"#34a853"},label:cjBasics.lang.i18n("cj_i18n_00139","Spreadsheets")},presentation:{icon:{name:"__mdi:drive_presentation",color:"#fbbc05"},label:cjBasics.lang.i18n("cj_i18n_00140","Presentations")},image:{icon:{name:"__mdi:drive_image",color:"#ea4335"},label:cjBasics.lang.i18n("cj_i18n_00141","Photos & images")},video:{icon:{name:"__mdi:drive_video",color:"#ea4335"},label:cjBasics.lang.i18n("cj_i18n_00142","Videos")}},N={priority:{label:cjBasics.lang.i18n("cj_i18n_06245","Priority"),iconName:"__mdi:priority_outline",newTabUrl:T+"priority"},"my-drive":{label:cjBasics.lang.i18n("cj_i18n_00001","My Drive"),iconName:"__mdi:my_drive",newTabUrl:T+"my-drive"},"shared-with-me":{label:cjBasics.lang.i18n("cj_i18n_00002","Shared with me"),iconName:"__mdi:people",newTabUrl:T+"shared-with-me"},recent:{label:cjBasics.lang.i18n("cj_i18n_00003","Recent"),iconName:"__mdi:schedule",newTabUrl:T+"recent"},starred:{label:cjBasics.lang.i18n("cj_i18n_00004","Starred"),iconName:"__mdi:star_outline",newTabUrl:T+"starred"},trash:{label:cjBasics.lang.i18n("cj_i18n_00005","Bin"),iconName:"__mdi:delete",newTabUrl:T+"trash"},computers:{label:cjBasics.lang.i18n("cj_i18n_01307","Computers"),iconName:"__mdi:devices",external:!0,newTabUrl:T+"computers"},backups:{label:cjBasics.lang.i18n("cj_i18n_01308","Backups"),iconName:"__mdi:mobile_friendly",external:!0,newTabUrl:T+"backups"},quota:{label:cjBasics.lang.i18n("cj_i18n_01833","Storage"),iconName:"__mdi:cloud",newTabUrl:T+"quota"},storage:{label:cjBasics.lang.i18n("cj_i18n_00077","Upgrade storage"),iconName:"__mdi:storage",external:!0,newTabUrl:a+"/"+l.wizPath+"settings/storage"}};function W(e){return"my-drive"===e||"priority"===e||"recent"===e||"starred"===e||"shared-with-me"===e}function L(){var i,e,a,n,c;U&&(d.cjceSetLoading(!0),c=J[v],i="shared-with-me"===b?{q:"trashed = false and not 'me' in owners and sharedWithMe",orderBy:"sharedWithMeTime desc"}:"recent"===b?{q:"(not mimeType = 'application/vnd.google-apps.folder') and trashed = false",orderBy:"recency desc"}:"trash"===b?{q:"trashed = true and explicitlyTrashed = true",orderBy:"folder,"+c}:"quota"===b?{q:"(not mimeType in 'application/vnd.google-apps.folder') and trashed = false and 'me' in owners",orderBy:"quotaBytesUsed desc"}:"starred"===b?{q:"trashed = false and starred = true",orderBy:"folder,"+c}:"my-drive"===b?{q:"trashed = false and 'root' in parents",orderBy:"folder,"+c}:"priority"===b?{priority:!0}:"folder"===b||"shareddrive"===b?{q:"trashed = false and '"+g.id+"' in parents",orderBy:"folder,"+c}:"search"===b?(c=["trashed = false"],-1!==(e=n=m.value).indexOf("type:")&&(e="",a=n.split("type:")[1],-1!==n.indexOf(" ")&&(e=n.split(" ")[1],a=a.split(" ")[0]),n=G[a],c.push(n)),c.push("fullText contains '"+e+"'"),{q:c.join(" and ")}):null,l.requirePage("doclist").then(function(){var e=cjce.createElement("cjg-doclist",{gAccount:l.account,darkMode:l.darkMode,additionalFragment:!i.priority&&p,listview:!i.priority&&y,shadow:"thinOnScroll",openInNew:l.openTab,onFolderClick:Z,onLoadingChange:function(e){d.cjceSetLoading(e)},searching:"search"===b,cachingId:W(b)?"bm_cache_v33__drive__mainviewcache_"+b:null,orderBy:i.orderBy,priority:i.priority,q:i.q});cjce.replaceChild(o,e,r),r=e}))}function k(e,i){var a=N[e],a=(_.hidden="folder"!==e,"search"!==b&&"folder"!==b&&("folder"===e?(j=b,h=g):"search"===e&&(c=b,n=g)),b=e,g=i,s&&(s.cjceSelectedId=e),a?d.cjceSetPageLabel(a.label):"folder"===b?(_.cjceSetPageLabel(""),function t(s){var e=cjBasics.urlParams.attach(S+"files/"+s.id,{corpora:"allDrives",fields:"name,id,parents,driveId",prettyPrint:"false",supportsAllDrives:"true",includeItemsFromAllDrives:"true"});cjgApis.request(e,{fetchAs:"json"},{account:l.account,requiredScopes:x}).then(function(i){var e,a,n=ae.createElement("span"),c=i.name||cjBasics.lang.i18n("cj_i18n_01794","Folder"),r=i.driveId===i.id,o=(r&&(c=D[i.id]||cjBasics.lang.i18n("cj_i18n_01795","Shared drive")),n.textContent=c,_.querySelector(".bm-p-drive-foldertree"));o?(e=ae.createTextNode(" > "),n.addEventListener("click",function(e){e.currentTarget===o.firstChild?r?k("shareddrive",{id:i.id,label:c}):k("my-drive"):k("folder",s)}),a=o.firstChild,o.insertBefore(e,a),o.insertBefore(n,e)):((o=ae.createElement("span")).classList.add("bm-p-drive-foldertree"),o.appendChild(n),_.cjceSetPageLabel(o)),"parents"in i&&t({id:i.parents[0]})})}(g)):"shareddrive"===b&&(s.cjceSelectedId="shareddrive-"+i.id,d.cjceSetPageLabel(i.label)),-1===A.indexOf(e));f.style.display=a?"none":"",L(),W(e)&&browserStorage.local.setItem("bm_pref__drive__lastid",e)}function Z(e){var i=e.folderColorRgb;k("folder",{label:e.name,id:e.id,color:i="#8f8f8f"===i?null:i})}function Q(e){var i=m.value;-1!==i.indexOf("type:")&&(i=i.split(" ")[1]),m.value="type:"+e+(i?" "+i:""),d.cjceSetNavigatorCollapseState(!0),"search"===b?L():k("search")}function z(e){e=e;var i="folder"!==b&&"shareddrive"!==b?e:(e=new URL(e),(i=new URLSearchParams(e.search)).set("folder",g.id),e.search=i,e.href);l.openTab(i)}function Y(){i=cjce.createElement("cjmd-sheet",{from:"bottom",triggerElement:u});var e=cjce.createElement("bm-shortcutlist",{onNewTab:function(e){"__bm_drive_upload"===e?E.upload.onClick():"__bm_drive_folder"===e&&E.folder.onClick()},items:U?[E.folder,E.upload]:[]}),e=(i.appendChild(e),cjce.createElement("cjmd-subheader",{color:!0,label:cjBasics.lang.i18n("cj_i18n_01309","Google apps")})),e=(i.appendChild(e),cjce.createElement("bm-shortcutlist",{onNewTab:z,items:V})),n=(i.appendChild(e),ae.createElement("div"));function a(e){n.textContent="";var i="NOT_AUTHORIZED"===e,a=Array.isArray(e)&&0<e.length;(i||a)&&(a=cjce.createElement("cjmd-subheader",{color:!0,label:cjBasics.lang.i18n("cj_i18n_01310","More apps")}),n.appendChild(a),i?(a=cjce.createElement("bm-inlinepermission",{description:cjBasics.lang.i18n("cj_i18n_01311","Grant read-only access to your Drive Apps so we can list more apps here"),onClick:function(){cjgApis.auth.requestPermissions(l.account,I)}}),n.appendChild(a)):(i=cjce.createElement("bm-shortcutlist",{onNewTab:z,items:e}),n.appendChild(i)))}i.appendChild(n);var c=null,r=!1;cjgApis.cache.getItem(l.account,"bm_cache_v33__drive__createapps").then(function(e){!r&&e&&(c=JSON.stringify(e),a(e))}),e=cjBasics.urlParams.attach(C+"/drive/v2/apps",{prettyPrint:"false",hl:cjBasics.lang.current,fields:"items(supportsCreate,id,name,createUrl,icons(size,category,iconUrl))"}),cjgApis.request(e,{fetchAs:"json"},{account:l.account,pa:"optional",key:M,requiredScopes:I}).then(function(e){var n=[];return e.items.forEach(function(e){var i,a;e.supportsCreate&&-1===F.indexOf(e.id)&&(a=24*ie.devicePixelRatio,e.icons.sort(function(e,i){return e.size<i.size?1:-1}).forEach(function(e){"application"===e.category&&e.size>=a&&(i=e.iconUrl)}),n.push({label:e.name,newTabUrl:e.createUrl,icon:{url:i}}))}),n},function(){return"NOT_AUTHORIZED"}).then(function(e){r=!0;var i=JSON.stringify(e);c&&i===c||(a(e),cjgApis.cache.setItem(l.account,"bm_cache_v33__drive__createapps",e))}),o.appendChild(i)}function K(e){i||Y(),setTimeout(function(){i.cjceToggleState(!0)}),setTimeout(function(){var e=i.querySelector(".cjmd-item");e&&e.focus()},50)}function X(c){return n=c,new Promise(function(e,i){var a=new FileReader;a.readAsBinaryString(n),a.addEventListener("load",function(){e(a.result)}),a.addEventListener("error",function(){var e=new Error("Read as binary failed");i(e)})}).then(function(e){var i=c.type||"application/octet-stream",a={name:c.name,mimeType:i},n=("folder"!==b&&"shareddrive"!==b||(a.parents=[g.id]),"bmfg_upload_boundary"),a=["--"+n,"Content-Type: application/json; charset=UTF-8",'Content-Disposition: form-data; name="metadata"',"",JSON.stringify(a),"--"+n,"Content-Type: "+i,'Content-Disposition: form-data; name="file"',"",e,"--"+n+"--"].join("\n"),i=cjBasics.urlParams.attach(C+"/upload/drive/v3/files",{uploadType:"multipart",prettyPrint:"false",fields:""});return cjgApis.request(i,{method:"POST",body:a,headers:{"Content-Type":'multipart/related; boundary="'+n+'"'}},{account:l.account,requiredScopes:x})},function(){});var n}function $(){u=cjce.createElement("gmd-fab",{label:cjBasics.lang.i18n("cj_i18n_00090","New file"),isPlaceholder:!0,onClick:K});var e=cjce.createElement("ccbm-fabgroup",{items:["main","upload","folder","document","spreadsheet","presentation","form"].map(function(e){var i,a,n;return"main"===e?u:(i="upload"===e||"folder"===e,n=E[e],a=cjce.createElement("ccbm-fab",{mini:!0,label:n.fabLabel,color:n.icon.color,fill:!i&&l.darkMode,iconName:n.icon.name,onClick:n.onClick||function(){z(n.newTabUrl)}}),i&&O.then(function(){U||a.parentNode.remove()}),a)})});o.appendChild(e)}function q(e){e=cjBasics.urlParams.attach(T+"search",{q:e});l.openTab(e)}function P(e){t.textContent="";var i=["priority","my-drive","shared-with-me","recent","starred","trash","quota"].map(function(e){var i=N[e];return i.id=e,i}),a=i.length-1,n=i[a],c=(i[a]=Object.assign({divider:!0},n),e.length);if(0<c){i.push({header:!0,color:!0,label:cjBasics.lang.i18n("cj_i18n_01782","Shared drives")});for(var r=0;r<c;r++){var o=e[r];D[o.id]=o.name,i.push({id:"shareddrive-"+o.id,iconName:"__mdi:cjg_blank_drive",label:o.name,sharedDriveId:o.id,newTabUrl:T+"folders/"+o.id,divider:r===c-1})}}["computers","backups","storage"].forEach(function(e){i.push(N[e])}),s=cjce.createElement("bm-navlist",{compact:!0,isSelector:!0,selectedId:b,items:i,onChange:function(e,i){U&&(i.sharedDriveId?k("shareddrive",{label:i.label,id:i.sharedDriveId}):k(e))},onNewTab:function(e,i){l.openTab(i.newTabUrl)}}),t.appendChild(s)}function ee(){if(!l.account.validSession)return P([]);var e,i=!1,a="";cjgApis.cache.getItem(l.account,"bm_cache_v33__drive__shareddrives").then(function(e){i||(e=e||[],a=JSON.stringify(e),P(e))}),e=cjBasics.urlParams.attach(S+"drives",{prettyPrint:"false",fields:"drives(id,name)",pageSize:"100"}),cjgApis.request(e,{fetchAs:"json"},{account:l.account,requiredScopes:x}).then(function(e){e=e.drives;return cjgApis.cache.setItem(l.account,"bm_cache_v33__drive__shareddrives",e),e},function(){return[]}).then(function(e){i=!0,e=e||[],JSON.stringify(e)!==a&&P(e)})}(B=ae.createElement("input")).type="file",B.multiple=!0,B.style.display="none",B.addEventListener("change",function(){d.cjceSetLoading(!0);for(var e=[],i=0;i<B.files.length;i++)e.push(X(B.files[i]));Promise.all(e).catch(function(){}).then(function(){d.cjceSetLoading(!1),L()})}),e=B,o.appendChild(e),d=cjce.createElement("bm-ogb",{loading:!0,drawer:!0,serviceIcon:"drive",bmApis:l,onNewTab:w,pageLabel:N[b].label,serviceLabel:cjBasics.lang.i18n("cj_i18n_00390","Drive"),searchbox:{onClear:function(){k(c,n)},onSubmit:function(e){if(!U)return q(e);"search"!==b?k("search"):L()},onMetaSubmit:q}}),t=d.cjceDrawerEl,ee(),(m=d.cjceSearchboxEl).classList.add("bm-p-drive-searchbox"),o.appendChild(d),l.setOnPageDisplayHandler(function(i){m.select(),setTimeout(function(){var e=i&&i.folderParameters;e&&Z(e)},300)}),$(),Promise.all([browserStorage.local.getItem("bm_pref__drive__lastid"),browserStorage.sync.getItem("bm_pref__drive__thumbs"),browserStorage.sync.getItem("bm_pref__drive__order")]).then(function(e){var i,a,n=e[0],c=e[1];function r(e){y=!y,i.hidden=y,a.hidden=!y,browserStorage.sync.setItem("bm_pref__drive__thumbs",!y),L()}e=e[2],v=e in R?e:"modifiedTime desc"===e?"lastModified":"modifiedByMeTime desc"===e?"lastModifiedByMe":"viewedByMeTime desc"===e?"lastOpenedByMe":"name",b=n||"my-drive",d.cjceSetPageLabel(N[b].label),y=!c,(e=cjce.createElement("cjmd-appbar")).classList.add("bm-p-drive-filterbar"),f=cjce.createElement("ccbm-iconbuttonmenu",{open:"end",label:cjBasics.lang.i18n("cj_i18n_00096","Order by"),iconName:"__mdi:cj_sort_by_alpha",items:R,selectedId:v,isSelector:!0,onChange:function(e){v=e,L(),browserStorage.sync.setItem("bm_pref__drive__order",e)}}),e.appendChild(f),(i=cjce.createElement("ccbm-iconbutton",{label:cjBasics.lang.i18n("cj_i18n_00106","Change view"),iconName:"__mdi:view_list",onClick:r})).hidden=y,e.appendChild(i),(a=cjce.createElement("ccbm-iconbutton",{label:cjBasics.lang.i18n("cj_i18n_00106","Change view"),iconName:"__mdi:view_module",onClick:r})).hidden=!y,e.appendChild(a),p=e,(_=cjce.createElement("bm-ogb",{displayBack:!0,floating:!0,aboveTopDrawer:!0,bmApis:l,onNewTab:w,serviceLabel:cjBasics.lang.i18n("cj_i18n_00390","Drive"),pageLabel:N["my-drive"].label,onBack:function(){k(j,h)}})).hidden=!0,o.appendChild(_);n=cjce.createElement("ccbm-menu",{variant:"searchbox",items:H,onClick:Q}),n.classList.add("bm-p-drive-searchmenu"),c=cjce.createElement("ccbm-iconbutton",{iconName:"__mdi:tune"});cjce.applyTemplate(c,"basic-togglemenuhandler",{menuElement:n}),c.classList.add("bm-p-drive-searchmenubutton"),m.appendChild(c),m.appendChild(n),L(),s.cjceSelectedId=b,ie.requestIdleCallback&&ie.requestIdleCallback(Y)}),O.then(function(e){var i,a,n;U||(cjgApis.cache.remove(l.account,["bm_cache_v33__drive__shareddrives","bm_cache_v33__drive__createapps","bm_cache_v33__drive__mainviewcache_shared-with-me","bm_cache_v33__drive__mainviewcache_recent","bm_cache_v33__drive__mainviewcache_trash","bm_cache_v33__drive__mainviewcache_starred","bm_cache_v33__drive__mainviewcache_my-drive","bm_cache_v33__drive__mainviewcache_priority"]),i=cjce.createElement("bm-ogb",{serviceIcon:"drive",bmApis:l,onNewTab:w,pageLabel:cjBasics.lang.i18n("cj_i18n_01167","Links"),serviceLabel:cjBasics.lang.i18n("cj_i18n_00390","Drive"),searchbox:{onSubmit:q,submitInNewTab:!0}}),cjce.replaceChild(o,i,d),(m=i.cjceSearchboxEl).focus(),i=cjce.createElement("cjmd-container",{shadow:"thinOnScroll",fabPadding:!0,scrollable:!0}),a=cjce.createElement("bm-inlinepermission",{description:cjBasics.lang.i18n("cj_i18n_01313","Grant access to your Google Drive Account so we can list your files and folders here"),divider:!0,onClick:function(){cjgApis.auth.requestPermissions(l.account,e.cjg_missing_scopes)}}),i.appendChild(a),a=Object.keys(N).map(function(e){e=N[e];return e.external=!0,e}),(n=(s=cjce.createElement("bm-navlist",{compact:!0,items:a,onNewTab:function(e,i){l.openTab(i.newTabUrl)}})).firstChild).hidden=!0,cjgApis.info.getUserDomain(l.account).then(function(e){e&&(n.hidden=!1)}),i.appendChild(s),cjce.replaceChild(o,i,r))})})}();