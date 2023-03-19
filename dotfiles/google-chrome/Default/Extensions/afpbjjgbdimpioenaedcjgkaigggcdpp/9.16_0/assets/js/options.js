function aload(l, cb) {
  var d = document, s = 'script', w = window;
  function go(){
    var js = d.createElement(s);
    cb = cb || function() {};
    //- check if exists
    //- if (d.getElementById(id)) return;
    js.onload = cb;
    js.src = l;
    try{d.head.appendChild(js)}catch(e){
      document.documentElement.appendChild(js);
    };
  }
  go();
};
$(function() {
  (function () {
    if (typeof EventTarget !== "undefined") {
        let func = EventTarget.prototype.addEventListener;
        EventTarget.prototype.addEventListener = function (type, fn, capture) {
            this.func = func;
            if(typeof capture !== "boolean"){
                capture = capture || {};
                capture.passive = false;
            }
            this.func(type, fn, capture);
        };
    };
  }());
  var QQRV = chrome.app.getDetails().version;
  if(localStorage.getItem('adv-config-fb') != "0") {
    window.fbAsyncInit = function() {
      FB.init({
        appId            : '1287552678098019',
        autoLogAppEvents : true,
        xfbml            : true,
        version          : 'v5.0'
      });
    };
    aload('https://connect.facebook.net/en_US/sdk.js');
  } else {
    $('#fbifm').remove();
    $('label').css('margin-top','16px');
    $('.checkbox').css('margin-bottom','57px');
    $('.checkbox label').css('margin-top','0');
  }
  $('[i18n-w]').each(function(){
    var m = $(this).attr('i18n-w');
    var t = '';
    t = chrome.i18n.getMessage(m);
    if(t.length > 0) {
      $(this).html(t);
    }
  })
  document.title = chrome.i18n.getMessage('app_name');
  if(!location.hash.includes('#pop')) {
    $('#advanced_features').attr('href', 'advanced_features.html')
  }
  $("#version_span").text(QQRV);
  var config_obj = JSON.parse(localStorage.getItem('config')) || {};
  var fore_bg = config_obj.fore_bg || '#000000';
  var back_bg = config_obj.back_bg || '#ffffff';
  var size = config_obj.size;
  var is_transparnet_bg = config_obj.is_transparnet_bg;

  $('#cp1').colorpicker({'color': fore_bg, format: 'hex'});
  $('#cp2').colorpicker({'color': back_bg, format: 'hex'});
  $('#img-size').val(size);
  $('#transbg').prop('checked', is_transparnet_bg);
  toggleCp2(is_transparnet_bg)
  $('#transbg').change(function(){
    var is_transparnet_bg = $('#transbg').prop('checked');
    toggleCp2(is_transparnet_bg)
  });
  function toggleCp2(transparnet_bg) {
    if (transparnet_bg) {
      $("#cp2").colorpicker('disable');
      $("#cp2").find('.input-group-addon, i').addClass('input-not-allowed');
      $("#cp2 input").addClass('hqr-transbg')
    } else {
      $("#cp2").colorpicker('enable');
      $("#cp2").find('.input-group-addon, i').removeClass('input-not-allowed');
      $("#cp2 input").removeClass('hqr-transbg')
    }
  }
  $('#show-form').click(function(){
    // $(this).text('Please fill in the form below');
    // $(this).addClass('done-show-form');
    // $('iframe').show();
    window.open('https://docs.google.com/forms/d/e/1FAIpQLSe3uPq9Tp7MIatsvo7cx1l-IzbadnB1wJywBqzIAv9FAs-a1Q/viewform', '_blank')
  })
  $('#save').click(function(){
    var fore_bg = $("#cp1").colorpicker('getValue');
    var back_bg = $("#cp2").colorpicker('getValue');
    var size = $('#img-size').val();
    if(size == "" || isNaN(size)) size = 300;
    size = Math.max(256, size);
    size = Math.min(1024, size);
    $('#img-size').val(size);
    var is_transparnet_bg = $('#transbg').prop('checked');
    var obj = {};
    obj.fore_bg = fore_bg;
    obj.back_bg = back_bg;
    obj.size = size;
    obj.is_transparnet_bg = is_transparnet_bg;
    var config_str = (JSON.stringify(obj));
    localStorage.setItem('config', config_str);
    $('.saved_bubble').remove();
    $('#save').after('<span class="saved_bubble">'+chrome.i18n.getMessage('saved')+'</span>');
    $('.saved_bubble').show();
    $('.saved_bubble').fadeOut('slow');
  })
  var hashes = location.hash;
  if(hashes.includes('pop')){
    $('#back-form').show();
    $('body').addClass('body-pop');
    $('#hotkey-settings').show();
  } else if (hashes.includes('#fixed,')) {
    var hash_legacy = location.href.split('#fixed,')[1];
    $('#back-form').attr('href', 'popup.html#fixed,' + hash_legacy);
    $('#advanced_features').attr('href', 'advanced_features.html#fixed,' + hash_legacy)
    $('#back-form').show();
    $('body').addClass('body-pop').css('width','auto');
    $('#hotkey-settings').show();
  } else {
    $('#show-form').show();
    $('.facebook-page').show();
    $('p.adj-opt').addClass('adj-center');
  }
  $('#hotkey-settings').click(function(){
    chrome.tabs.create({url: 'chrome://extensions/configureCommands'});
  })
  chrome.extension.sendMessage({pop: QQRV+'/pop/settings'}, function(response) {
  });
  $('#donate a').click(()=>{
    prompt('Address:',`bitcoin:19B8n3aB81aFyDuFBLxnhFmBhyrseXP28n`);
  })
});