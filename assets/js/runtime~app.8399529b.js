(()=>{"use strict";var e,r,t,a,n,o,i,s={},l={};function d(e){var r=l[e];if(void 0!==r)return r.exports;var t=l[e]={id:e,loaded:!1,exports:{}};return s[e].call(t.exports,t,t.exports,d),t.loaded=!0,t.exports}d.m=s,e="function"==typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",r="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",t="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",a=e=>{e&&e.d<1&&(e.d=1,e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},d.a=(n,o,i)=>{var s;i&&((s=[]).d=-1);var l,d,u,p=new Set,c=n.exports,f=new Promise(((e,r)=>{u=r,d=e}));f[r]=c,f[e]=e=>(s&&e(s),p.forEach(e),f.catch((e=>{}))),n.exports=f,o((n=>{var o;l=(n=>n.map((n=>{if(null!==n&&"object"==typeof n){if(n[e])return n;if(n.then){var o=[];o.d=0,n.then((e=>{i[r]=e,a(o)}),(e=>{i[t]=e,a(o)}));var i={};return i[e]=e=>e(o),i}}var s={};return s[e]=e=>{},s[r]=n,s})))(n);var i=()=>l.map((e=>{if(e[t])throw e[t];return e[r]})),d=new Promise((r=>{(o=()=>r(i)).r=0;var t=e=>e!==s&&!p.has(e)&&(p.add(e),e&&!e.d&&(o.r++,e.push(o)));l.map((r=>r[e](t)))}));return o.r?d:i()}),(e=>(e?u(f[t]=e):d(c),a(s)))),s&&s.d<0&&(s.d=0)},n=[],d.O=(e,r,t,a)=>{if(!r){var o=1/0;for(u=0;u<n.length;u++){for(var[r,t,a]=n[u],i=!0,s=0;s<r.length;s++)(!1&a||o>=a)&&Object.keys(d.O).every((e=>d.O[e](r[s])))?r.splice(s--,1):(i=!1,a<o&&(o=a));if(i){n.splice(u--,1);var l=t();void 0!==l&&(e=l)}}return e}a=a||0;for(var u=n.length;u>0&&n[u-1][2]>a;u--)n[u]=n[u-1];n[u]=[r,t,a]},d.n=e=>{var r=e&&e.__esModule?()=>e.default:()=>e;return d.d(r,{a:r}),r},d.d=(e,r)=>{for(var t in r)d.o(r,t)&&!d.o(e,t)&&Object.defineProperty(e,t,{enumerable:!0,get:r[t]})},d.f={},d.e=e=>Promise.all(Object.keys(d.f).reduce(((r,t)=>(d.f[t](e,r),r)),[])),d.u=e=>"assets/js/"+{94:"guide.html",470:"index.html",490:"404.html",985:"randomize.html"}[e]+"."+{94:"06272205",470:"8e6fa245",490:"3eec13e9",985:"4bdaf1be"}[e]+".js",d.miniCssF=e=>{},d.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),d.o=(e,r)=>Object.prototype.hasOwnProperty.call(e,r),o={},i="web:",d.l=(e,r,t,a)=>{if(o[e])o[e].push(r);else{var n,s;if(void 0!==t)for(var l=document.getElementsByTagName("script"),u=0;u<l.length;u++){var p=l[u];if(p.getAttribute("src")==e||p.getAttribute("data-webpack")==i+t){n=p;break}}n||(s=!0,(n=document.createElement("script")).charset="utf-8",n.timeout=120,d.nc&&n.setAttribute("nonce",d.nc),n.setAttribute("data-webpack",i+t),n.src=e),o[e]=[r];var c=(r,t)=>{n.onerror=n.onload=null,clearTimeout(f);var a=o[e];if(delete o[e],n.parentNode&&n.parentNode.removeChild(n),a&&a.forEach((e=>e(t))),r)return r(t)},f=setTimeout(c.bind(null,void 0,{type:"timeout",target:n}),12e4);n.onerror=c.bind(null,n.onerror),n.onload=c.bind(null,n.onload),s&&document.head.appendChild(n)}},d.r=e=>{"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},d.v=(e,r,t,a)=>{var n=fetch(d.p+""+t+".module.wasm"),o=()=>n.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,a))).then((r=>Object.assign(e,r.instance.exports)));return n.then((r=>"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(r,a).then((r=>Object.assign(e,r.instance.exports)),(e=>{if("application/wasm"!==r.headers.get("Content-Type"))return console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",e),o();throw e})):o()))},d.p="/spritzer/",(()=>{var e={750:0,118:0};d.f.j=(r,t)=>{var a=d.o(e,r)?e[r]:void 0;if(0!==a)if(a)t.push(a[2]);else if(/^(118|750)$/.test(r))e[r]=0;else{var n=new Promise(((t,n)=>a=e[r]=[t,n]));t.push(a[2]=n);var o=d.p+d.u(r),i=new Error;d.l(o,(t=>{if(d.o(e,r)&&(0!==(a=e[r])&&(e[r]=void 0),a)){var n=t&&("load"===t.type?"missing":t.type),o=t&&t.target&&t.target.src;i.message="Loading chunk "+r+" failed.\n("+n+": "+o+")",i.name="ChunkLoadError",i.type=n,i.request=o,a[1](i)}}),"chunk-"+r,r)}},d.O.j=r=>0===e[r];var r=(r,t)=>{var a,n,[o,i,s]=t,l=0;if(o.some((r=>0!==e[r]))){for(a in i)d.o(i,a)&&(d.m[a]=i[a]);if(s)var u=s(d)}for(r&&r(t);l<o.length;l++)n=o[l],d.o(e,n)&&e[n]&&e[n][0](),e[n]=0;return d.O(u)},t=self.webpackChunkweb=self.webpackChunkweb||[];t.forEach(r.bind(null,0)),t.push=r.bind(null,t.push.bind(t))})()})();