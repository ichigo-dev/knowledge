var QR_SIZE = 600;
var fore_bg;
var back_bg;
var img_size;
var is_transparnet_bg;
var QQRV = chrome.app.getDetails().version;
window.FIXED_POPUP = false;
$(function () {
  if(localStorage.getItem('_r5') != '1') {
    $('#rate5').show();
    $('#qrcode-href').css('height','36px');
    trackContent(QQRV+'/r5');
  }
  $('#rate5').css({
    "border-radius": "50px",
    "font-size": "20px"
  }).click(function(){
    $('#hide_reviews').click();
    $(this).remove();
    localStorage.setItem('_r5', '1');
  });

  $('[i18n-w]').each(function(){
    var m = $(this).attr('i18n-w');
    var t = '';
    t = chrome.i18n.getMessage(m);
    if(t.length > 0) {
      $(this).html(t);
    }
  })
  document.title = chrome.i18n.getMessage('app_name');
  if(chrome.i18n.getUILanguage() == 'ja') {
    $('head').append('<style>.high-qr a{color:#333;padding: 12px 8px 2px;border-radius:18px;text-decoration:none;margin:0 2px}</style>')
  }
  if(chrome.i18n.getUILanguage().includes('zh')) {
    $('head').append('<style>.high-qr a{margin:0 2px}.high-qr a span{transform:scale(0.833);.high-qr span{margin-top:0}</style>')
  }
  var config_obj = JSON.parse(localStorage.getItem('config')) || {};
  fore_bg = config_obj.fore_bg || '#000000';
  back_bg = config_obj.back_bg || '#ffffff';
  img_size = config_obj.size || 600;
  is_transparnet_bg = config_obj.is_transparnet_bg;
  
  setTimeout(function(){
    $('#qrcode-href').css('visibility', 'visible').focus().select();
  },99);

  chrome.tabs.query({active:true, currentWindow:true}, updateContentByTabs);
  $("#qrcode-regenerate").click(function(){
    renderQRHandler();
    if (window.FIXED_POPUP) {return}
    $('#qrcode-img').css('opacity',0);
    $('#qrcode-img').animate({'opacity':1},222,function (t) { return t*(2-t) });
  });
  $('#version').text(chrome.app.getDetails().version);
  $('#credit_get').click(function(){
    trackContent(QQRV+'/pop/get_this');
  })
  $('#credit_support').click(function(){
    trackContent(QQRV+'/pop/support');
  })
  $('#export').click(function(){
    trackContent(QQRV+'/pop/export');
  })
  $('.hd').click(function(){
    trackContent(QQRV+'/pop/hd');
  })
  $('.withlogo').click(function(){
    trackContent(QQRV+'/pop/withlogo');
  })
  $('#qrcode-regenerate').click(function(){
    trackContent(QQRV+'/pop/regenerate');
  })

  $('#fixed_pop, #fixed_print').click(function(){
    var id = $(this).attr('id');
    trackContent(QQRV+'/pop/'+id);
    var pop_width = 370; //$(window).width();
    var pop_height = 608; // $(window).height() + 29;
    var left = (screen.width - pop_width) * .97;
    var top = 50;
    var windowName = 'popUpWindow';
    if(localStorage.getItem('adv-config-m_fixed') == '1') {
      windowName += Math.random();
    } 
    window.open(location.href + '#'+id+','+$('#qrcode-href').val(), 
      windowName,
      'scrollbars=no,resizable=yes,titlebar=no,toolbar=no,'
      + 'menubar=no,location=no,directories=no,status=no,'
      + 'width=' 
      + pop_width + ',height=' + pop_height + ',top=' + top + ', left=' + left);
  })
  // Not firing when paste by mouse.
  $('#qrcode-href').on('keydown keyup input contextmenu', function(e){
    renderQRHandler();
  })

  if(window.location.search.includes('?c=')){
    $('body').css('margin', '0 auto');
    $('body').css('height', 'auto');
    $('#settings').hide();
    $('#ctxt-only').show();
    $('.popup-footer-right').appendTo('body').addClass('ctxt-only-style');
    var c = window.location.search.split('?c=')[1];
    c = decodeURIComponent(c);
    var check = setInterval(function(){
      if($('#qrcode-href')){
        $('#qrcode-href').val(c);
        $('#qrcode-regenerate').click();
        clearInterval(check);        
      }
    }, 99);
  }

  // Check if the popup is in fixed mode
  if (!window.location.href.includes('#fixed_')) {
    $('#fixed_pop, #fixed_print').show();
  } else {
    // In a fixed up mode
    $('body').css('width', 'auto');
    var qr_string = location.href.split('#fixed,')[1];
    $('#settings').attr('href', 'options.html#fixed,'+qr_string);
    window.FIXED_POPUP = true;
    $('#qrcode-href').val(qr_string);
    var check = setInterval(function(){
      if($('#qrcode-href')){
        $('#qrcode-href').val(qr_string);
        $('#qrcode-regenerate').click();
        clearInterval(check);
        setTimeout(()=>{
          if(window.location.href.includes('#fixed_print')) {
            printThis();
          }   

        },9)
      }
    }, 99);
  }

  removeStatusBar()
});

function printThis(){
  var $img = $('#qrcode-img img');
  $img.prependTo('body');
  var width = window.outerWidth;
  var height = window.outerHeight;
  window.resizeTo(800,600);
  $('body').append(`
    <style id="print-css">
        @page {
          size: auto;
          margin: 0mm;
        }
        body {
          background-color:#FFFFFF;
          margin: 0px;
        }
        #qrcode-img {
          margin-top:100px;
        }
        #qrcode-href,
        a, button {
          display:none !important
        }
        body img {
          max-width: 66%;
          display: block;
          position:absolute;
          left:50%;
          top:50%;
          transform:translate(-50%,-50%);
          margin:9px auto;
        }
        body, html {
          height:auto !important;
        }
    </style>
  `);
  window.print();
  window.close();
  window.resizeTo(width,height);
  $img.prependTo('#qrcode-img');
  $('#print-css').remove();
}

function renderQRHandler(){
  var href = $("#qrcode-href").val();
  var $qrcode_img = $('#qrcode-img');
  $qrcode_img.html('');
  renderQR($qrcode_img, img_size, href);
  updateImgHref();
}
function updateContentByTabs(tabs) {
    var href = tabs[0].url;
    var title = tabs[0].title;
    var $qrcode_img = $('#qrcode-img');
    setTimeout(function() {
      $qrcode_img.html('');
      renderQR($qrcode_img, img_size, href);
      updateImgHref();
    }, 20);
    $('#qrcode-href').val(href);
}
function renderQR($el, the_size, the_text){
  var quiet = '1';
  if(back_bg != '#ffffff') {
    quiet = '1';
  }
  if (is_transparnet_bg) {
    back_bg = null;
  }
  $el.qrcode(qrObjectBuilder(the_size, fore_bg, the_text, back_bg,quiet));
  $('#qrcode-img-buffer').empty().qrcode(qrObjectBuilder(the_size, fore_bg, the_text, back_bg,1, true));
}
function qrObjectBuilder(s,f,t,b,q,c){
  var r = 'image';
  if (c) {
    r = 'canvas';
  }
  var o = {
    'render':r,
    size:s,
    fill:f,
    text:t,
    background:b,
    'quiet':q
  }
  o.ecLevel = 'L';
  return o;
}
function updateImgHref() {
  var link = $("#export")[0];
  link.download = 'exported_qrcode_image_'+img_size+'.png';
  link.href = $('#qrcode-img-buffer > canvas')[0].toDataURL();
}

var init_text = chrome.i18n.getMessage('saveimg');
$('body').on('mouseover', '#export', function () {
  var $svg = $(this).find('svg');
  $svg.find('#svg_text').text(img_size);
  $svg.find('#toggle_arrow').hide();

  // $(this).text(init_text + ' (' + img_size + 'x' + img_size + ')');
});
$('body').on('mouseout', '#export', function () {
  // $(this).text(init_text);
  var $svg = $(this).find('svg');
  $svg.find('#svg_text').text('');
  $svg.find('#toggle_arrow').show();
});

$('body').on('mouseover', '.withlogo', function () {
  var $svg = $(this).find('svg');
  $svg.find('#svg_logo_anchor').attr('transform', 'matrix( 1, 0, 0, 1, -506,-236)');
});
$('body').on('mouseout', '.withlogo', function () {
  var $svg = $(this).find('svg');
  $svg.find('#svg_logo_anchor').attr('transform', 'matrix( 1, 0, 0, 1, -536,-226)');
});

$('body').on('mouseover', '.hd', function () {
  var $svg = $(this).find('svg');
  $svg.find('#svg_hd_anchor').attr('transform', 'matrix( 1, 0, 0, 1, -303,-163)');
});
$('body').on('mouseout', '.hd', function () {
  var $svg = $(this).find('svg');
  $svg.find('#svg_hd_anchor').attr('transform', 'matrix( 1, 0, 0, 1, -313,-143)');
});


$('body').on('mouseover', '.bulk', function () {
  var $svg = $(this).find('svg');
  $svg.find('#svg_bulk_anchor').attr('transform', 'matrix( 1, 0, 0, 1, 0,10)');
});
$('body').on('mouseout', '.bulk', function () {
  var $svg = $(this).find('svg');
  $svg.find('#svg_bulk_anchor').attr('transform', 'matrix( 1, 0, 0, 1, 0,0)');
});

function removeStatusBar(){
  setTimeout(function(){
    $('a').each(function(){
      if(['fixed_pop','fixed_print','export'].includes($(this).attr('id'))) return;
      var reallink = $(this).attr('href');
      var is_blank = $(this).attr('target') == '_blank';
      $(this).attr('data-realhref', reallink)
             .css('cursor', 'pointer')
             .removeAttr('href');
      if (is_blank) {
        $(this).click(function(){
          var link = $(this).attr('data-realhref');
          window.open(link, '_blank');
        })
      } else {
        $(this).click(function(){
          var link = $(this).attr('data-realhref');
          location = link; 
        })
      }
    })
  },0)
}



function trackContent(c) {
  chrome.extension.sendMessage({pop: c}, function(response) {
  });
}