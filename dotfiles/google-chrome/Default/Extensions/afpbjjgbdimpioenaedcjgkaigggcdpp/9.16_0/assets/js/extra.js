var _AnalyticsCode = 'UA-27468792-5';
var extensionVersion = chrome.app.getDetails().version;
var _gaq = _gaq || [];
_gaq.push(['_setAccount', _AnalyticsCode]);
(function() {
  var ga = document.createElement('script');
  ga.type = 'text/javascript';
  ga.async = true;
  ga.src = 'https://ssl.google-analytics.com/ga.js';
  var s = document.getElementsByTagName('script')[0];
  s.parentNode.insertBefore(ga, s);
})();
trackPage('');
function trackPage(p){
	p = p.substr(0, 100);
	var page = 'v_' + extensionVersion + "/" + p;
	_gaq.push(['_trackPageview', page]);
}

chrome.extension.onMessage.addListener(function(request, sender, sendResponse) {
  console.log('get:', request);
  if (request.hasOwnProperty('pop')) {
  	trackPage('popup/' + request.pop);
  }
  if (request.hasOwnProperty('opt')) {
    trackPage('opt/' + request.opt);
  }
  sendResponse();
  return true;
});