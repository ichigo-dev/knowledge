var MSG_DISABLE_CONTEXT_MENUS = 'A1';
var MSG_ENABLE_CONTEXT_MENUS = 'A2';
var MSG_ENABLE_SCANNER = 'A3';
$(function() {
  $('[i18n-w]').each(function(){
    var m = $(this).attr('i18n-w');
    var t = '';
    t = chrome.i18n.getMessage(m);
    if(t.length > 0) {
      $(this).html(t);
    }
  })
  document.title = chrome.i18n.getMessage('app_name');
  if(location.hash.includes('#pop')) {
    $('#back-form').attr('href', 'options.html#pop');
  }
  chrome.permissions.contains({
    permissions: ['tabs']
  }, function(result) {
    if (result) {
      // The extension has the permissions.
      $('#grant_button').remove();
      $('.tabs_permission').css('margin-bottom','32px')
    } else {
      // The extension doesn't have the permissions.
      $('.tabs_permission').addClass('disabled-form');
      $('#grant_button').click(function(){
        track('grant');
        // Tap is ON
        chrome.permissions.request({
          permissions:['tabs'],
          origins: ['\u003Call_urls>']
        }, function(granted) {
          if (granted) {
            location.reload();
          }
        });
      });
      return;
    }
  });
  function init_scanner_menu() {
    sender(MSG_ENABLE_SCANNER);
  }
  if (localStorage.getItem('adv-config-scanner') == "1") {
    $('#scanner_feature').prop('checked', true);
    init_scanner_menu();
  }
  $('#scanner_feature').change(function(){
    var enable_scanner = $('#scanner_feature').prop('checked');
    track('scanner:'+enable_scanner);
    if (enable_scanner) {
      // granted, set to one.
      localStorage.setItem('adv-config-scanner',"1");
      $('#scanner_feature').prop('checked', true);
      init_scanner_menu();
    } else {
      // Tap is OFF
      localStorage.setItem('adv-config-scanner',"0");
      try {
        chrome.contextMenus.remove(Number(localStorage.getItem('cmid_scan')));
        localStorage.removeItem('cmid_scan');
      } catch (e) {
      }
    }
  });

  if (localStorage.getItem('adv-config-context') != "0") {
    $('#context_menus').prop('checked', true);
  } else {
    $('#context_menus').prop('checked', false);
  }
  if (localStorage.getItem('adv-config-fb') != "0") {
    $('#hide_fb').prop('checked', true);
  } else {
    $('#hide_fb').prop('checked', false);
  }
  if (localStorage.getItem('adv-config-m_fixed') == "1") {
    $('#mfixed').prop('checked', true);
  } else {
    $('#mfixed').prop('checked', false);
  }
  $('#context_menus').change(function(){
    var enable_context_menu = $('#context_menus').prop('checked');
    track('context-menus:'+enable_context_menu);
    if (enable_context_menu) {
      localStorage.setItem('adv-config-context', '1');
      sender(MSG_ENABLE_CONTEXT_MENUS);
    } else {
      localStorage.setItem('adv-config-context', '0');
      sender(MSG_DISABLE_CONTEXT_MENUS);
    }
  });

  $('#hide_fb').change(function(){
    var enable_fb_box = $('#hide_fb').prop('checked');
    track('fb-box:'+enable_fb_box);
    if (enable_fb_box) {
      localStorage.setItem('adv-config-fb', '1');
      sender(MSG_ENABLE_CONTEXT_MENUS);
    } else {
      localStorage.setItem('adv-config-fb', '0');
      sender(MSG_DISABLE_CONTEXT_MENUS);
    }
  });

  $('#mfixed').change(function(){
    var m_fixed = $('#mfixed').prop('checked');
    track('m_fixed:'+m_fixed);
    if (m_fixed) {
      localStorage.setItem('adv-config-m_fixed', '1');
    } else {
      localStorage.setItem('adv-config-m_fixed', '0');
    }
  });
  function sender(msg){
    chrome.runtime.sendMessage({code: msg}, function(response) {
      console.log(response);
    });
  }
  function track(msg){
    chrome.runtime.sendMessage({opt: msg}, function(response) {
      console.log(response);
    });
  }
  if (location.hash.includes('#fixed,')) {
    var hash_legacy = location.href.split('#fixed,')[1];
    $('#back-form').attr('href', 'options.html#fixed,' + hash_legacy);
    $('body').css('width', 'auto');
  }
  $('#donate a').click(()=>{
    prompt('Address:',`bitcoin:19B8n3aB81aFyDuFBLxnhFmBhyrseXP28n`);
  })
});