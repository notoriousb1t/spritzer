"use strict";(self.webpackChunkweb=self.webpackChunkweb||[]).push([[524],{3650:(e,t,n)=>{n.a(e,(async(e,o)=>{try{n.d(t,{B1:()=>r.B1,Mi:()=>r.Mi,Ts:()=>r.Ts});var _=n(2554),r=n(7070),i=e([_]);_=(i.then?(await i)():i)[0],(0,r.lI)(_),o()}catch(e){o(e)}}))},7070:(e,t,n)=>{let o;function _(e){o=e}n.d(t,{$D:()=>j,$Z:()=>ce,B1:()=>y,BZ:()=>te,FT:()=>X,Fm:()=>ae,Gu:()=>Q,Kc:()=>_e,Ke:()=>N,Lo:()=>le,MF:()=>$,Mi:()=>m,Mx:()=>q,O6:()=>G,PR:()=>Z,Pf:()=>Y,Py:()=>be,Qn:()=>pe,Si:()=>J,Ts:()=>p,Uv:()=>H,V5:()=>B,Wv:()=>ue,Xu:()=>P,_J:()=>O,_m:()=>se,a0:()=>x,bk:()=>R,cX:()=>ne,cq:()=>ee,f1:()=>W,if:()=>fe,kh:()=>oe,lI:()=>_,mS:()=>I,qv:()=>K,u$:()=>D,uI:()=>M,v6:()=>de,vA:()=>re,vU:()=>ie,yc:()=>U,zQ:()=>V}),e=n.hmd(e);let r=new("undefined"==typeof TextDecoder?(0,e.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});r.decode();let i=null;function s(){return null!==i&&0!==i.byteLength||(i=new Uint8Array(o.memory.buffer)),i}function a(e,t){return e>>>=0,r.decode(s().subarray(e,e+t))}const l=new Array(128).fill(void 0);l.push(void 0,null,!0,!1);let c=l.length;function u(e){c===l.length&&l.push(l.length+1);const t=c;return c=l[t],l[t]=e,t}function d(e){return l[e]}function f(e){const t=d(e);return function(e){e<132||(l[e]=c,c=e)}(e),t}function p(){o.init()}let b=0;function g(e,t){const n=t(1*e.length,1)>>>0;return s().set(e,n/1),b=e.length,n}let w=null;function h(){return null!==w&&0!==w.byteLength||(w=new Int32Array(o.memory.buffer)),w}function m(e,t){try{const c=o.__wbindgen_add_to_stack_pointer(-16),u=g(e,o.__wbindgen_malloc),d=b;!function(e,t){if(!(e instanceof t))throw new Error(`expected instance of ${t.name}`);e.ptr}(t,C);var n=t.__destroy_into_raw();o.process_zelda3(c,u,d,n);var _=h()[c/4+0],r=h()[c/4+1],i=(a=_,l=r,a>>>=0,s().subarray(a/1,a/1+l)).slice();return o.__wbindgen_free(_,1*r,1),i}finally{o.__wbindgen_add_to_stack_pointer(16)}var a,l}function y(e){const t=g(e,o.__wbindgen_malloc),n=b,_=o.detect_options(t,n);return S.__wrap(_)}let k=new("undefined"==typeof TextEncoder?(0,e.require)("util").TextEncoder:TextEncoder)("utf-8");const v="function"==typeof k.encodeInto?function(e,t){return k.encodeInto(e,t)}:function(e,t){const n=k.encode(e);return t.set(n),{read:e.length,written:n.length}};function L(e,t,n){if(void 0===n){const n=k.encode(e),o=t(n.length,1)>>>0;return s().subarray(o,o+n.length).set(n),b=n.length,o}let o=e.length,_=t(o,1)>>>0;const r=s();let i=0;for(;i<o;i++){const t=e.charCodeAt(i);if(t>127)break;r[_+i]=t}if(i!==o){0!==i&&(e=e.slice(i)),_=n(_,o,o=i+3*e.length,1)>>>0;const t=s().subarray(_+i,_+o);i+=v(e,t).written,_=n(_,o,i,1)>>>0}return b=i,_}let z=null;function A(e,t){e>>>=0;const n=(null!==z&&0!==z.byteLength||(z=new Uint32Array(o.memory.buffer)),z).subarray(e/4,e/4+t),_=[];for(let e=0;e<n.length;e++)_.push(f(n[e]));return _}function F(e,t){try{return e.apply(this,t)}catch(e){o.__wbindgen_exn_store(u(e))}}const E="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry((e=>o.__wbg_detectoptionsresult_free(e>>>0)));class S{static __wrap(e){e>>>=0;const t=Object.create(S.prototype);return t.__wbg_ptr=e,E.register(t,t.__wbg_ptr,t),t}__destroy_into_raw(){const e=this.__wbg_ptr;return this.__wbg_ptr=0,E.unregister(this),e}free(){const e=this.__destroy_into_raw();o.__wbg_detectoptionsresult_free(e)}get supported(){return 0!==o.__wbg_get_detectoptionsresult_supported(this.__wbg_ptr)}set supported(e){o.__wbg_set_detectoptionsresult_supported(this.__wbg_ptr,e)}get game_type(){let e,t;try{const r=o.__wbindgen_add_to_stack_pointer(-16);o.detectoptionsresult_game_type(r,this.__wbg_ptr);var n=h()[r/4+0],_=h()[r/4+1];return e=n,t=_,a(n,_)}finally{o.__wbindgen_add_to_stack_pointer(16),o.__wbindgen_free(e,t,1)}}get options(){const e=o.detectoptionsresult_options(this.__wbg_ptr);return C.__wrap(e)}}const T="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry((e=>o.__wbg_z3wasmoptions_free(e>>>0)));class C{static __wrap(e){e>>>=0;const t=Object.create(C.prototype);return t.__wbg_ptr=e,T.register(t,t.__wbg_ptr,t),t}__destroy_into_raw(){const e=this.__wbg_ptr;return this.__wbg_ptr=0,T.unregister(this),e}free(){const e=this.__destroy_into_raw();o.__wbg_z3wasmoptions_free(e)}get boss_shuffle(){return 0!==o.__wbg_get_z3wasmoptions_boss_shuffle(this.__wbg_ptr)}set boss_shuffle(e){o.__wbg_set_z3wasmoptions_boss_shuffle(this.__wbg_ptr,e)}get killable_thieves(){return 0!==o.__wbg_get_z3wasmoptions_killable_thieves(this.__wbg_ptr)}set killable_thieves(e){o.__wbg_set_z3wasmoptions_killable_thieves(this.__wbg_ptr,e)}get mushroom_shuffle(){return 0!==o.__wbg_get_z3wasmoptions_mushroom_shuffle(this.__wbg_ptr)}set mushroom_shuffle(e){o.__wbg_set_z3wasmoptions_mushroom_shuffle(this.__wbg_ptr,e)}get shadow_bees(){return 0!==o.__wbg_get_z3wasmoptions_shadow_bees(this.__wbg_ptr)}set shadow_bees(e){o.__wbg_set_z3wasmoptions_shadow_bees(this.__wbg_ptr,e)}get seed(){let e,t;try{const r=o.__wbindgen_add_to_stack_pointer(-16);o.detectoptionsresult_game_type(r,this.__wbg_ptr);var n=h()[r/4+0],_=h()[r/4+1];return e=n,t=_,a(n,_)}finally{o.__wbindgen_add_to_stack_pointer(16),o.__wbindgen_free(e,t,1)}}set seed(e){const t=L(e,o.__wbindgen_malloc,o.__wbindgen_realloc),n=b;o.z3wasmoptions_set_seed(this.__wbg_ptr,t,n)}get ow_balancing(){let e,t;try{const r=o.__wbindgen_add_to_stack_pointer(-16);o.z3wasmoptions_ow_balancing(r,this.__wbg_ptr);var n=h()[r/4+0],_=h()[r/4+1];return e=n,t=_,a(n,_)}finally{o.__wbindgen_add_to_stack_pointer(16),o.__wbindgen_free(e,t,1)}}get ow_enemy_shuffle(){let e,t;try{const r=o.__wbindgen_add_to_stack_pointer(-16);o.z3wasmoptions_ow_enemy_shuffle(r,this.__wbg_ptr);var n=h()[r/4+0],_=h()[r/4+1];return e=n,t=_,a(n,_)}finally{o.__wbindgen_add_to_stack_pointer(16),o.__wbindgen_free(e,t,1)}}set ow_enemy_shuffle(e){const t=L(e,o.__wbindgen_malloc,o.__wbindgen_realloc),n=b;o.z3wasmoptions_set_ow_enemy_shuffle(this.__wbg_ptr,t,n)}set ow_balancing(e){const t=L(e,o.__wbindgen_malloc,o.__wbindgen_realloc),n=b;o.z3wasmoptions_set_ow_balancing(this.__wbg_ptr,t,n)}get uw_enemy_shuffle(){let e,t;try{const r=o.__wbindgen_add_to_stack_pointer(-16);o.z3wasmoptions_uw_enemy_shuffle(r,this.__wbg_ptr);var n=h()[r/4+0],_=h()[r/4+1];return e=n,t=_,a(n,_)}finally{o.__wbindgen_add_to_stack_pointer(16),o.__wbindgen_free(e,t,1)}}set uw_enemy_shuffle(e){const t=L(e,o.__wbindgen_malloc,o.__wbindgen_realloc),n=b;o.z3wasmoptions_set_uw_enemy_shuffle(this.__wbg_ptr,t,n)}get uw_balancing(){let e,t;try{const r=o.__wbindgen_add_to_stack_pointer(-16);o.z3wasmoptions_uw_balancing(r,this.__wbg_ptr);var n=h()[r/4+0],_=h()[r/4+1];return e=n,t=_,a(n,_)}finally{o.__wbindgen_add_to_stack_pointer(16),o.__wbindgen_free(e,t,1)}}set uw_balancing(e){const t=L(e,o.__wbindgen_malloc,o.__wbindgen_realloc),n=b;o.z3wasmoptions_set_uw_balancing(this.__wbg_ptr,t,n)}get balancing_options(){try{const _=o.__wbindgen_add_to_stack_pointer(-16);o.z3wasmoptions_balancing_options(_,this.__wbg_ptr);var e=h()[_/4+0],t=h()[_/4+1],n=A(e,t).slice();return o.__wbindgen_free(e,4*t,4),n}finally{o.__wbindgen_add_to_stack_pointer(16)}}get ow_enemy_shuffle_options(){try{const _=o.__wbindgen_add_to_stack_pointer(-16);o.z3wasmoptions_ow_enemy_shuffle_options(_,this.__wbg_ptr);var e=h()[_/4+0],t=h()[_/4+1],n=A(e,t).slice();return o.__wbindgen_free(e,4*t,4),n}finally{o.__wbindgen_add_to_stack_pointer(16)}}get uw_enemy_shuffle_options(){try{const _=o.__wbindgen_add_to_stack_pointer(-16);o.z3wasmoptions_uw_enemy_shuffle_options(_,this.__wbg_ptr);var e=h()[_/4+0],t=h()[_/4+1],n=A(e,t).slice();return o.__wbindgen_free(e,4*t,4),n}finally{o.__wbindgen_add_to_stack_pointer(16)}}}function U(e,t){return u(a(e,t))}function R(e){f(e)}function X(e,t,n,o){console.debug(d(e),d(t),d(n),d(o))}function I(e){console.error(d(e))}function O(e,t,n,o){console.error(d(e),d(t),d(n),d(o))}function x(e,t,n,o){console.info(d(e),d(t),d(n),d(o))}function M(e,t,n,o){console.log(d(e),d(t),d(n),d(o))}function W(e,t,n,o){console.warn(d(e),d(t),d(n),d(o))}function B(){return u(new Error)}function D(e,t){const n=L(d(t).stack,o.__wbindgen_malloc,o.__wbindgen_realloc),_=b;h()[e/4+1]=_,h()[e/4+0]=n}function P(e,t){let n,_;try{n=e,_=t,console.error(a(e,t))}finally{o.__wbindgen_free(n,_,1)}}function j(e){return u(d(e).crypto)}function K(e){const t=d(e);return"object"==typeof t&&null!==t}function q(e){return u(d(e).process)}function N(e){return u(d(e).versions)}function $(e){return u(d(e).node)}function Q(e){return"string"==typeof d(e)}function G(){return F((function(){return u(e.require)}),arguments)}function Z(e){return"function"==typeof d(e)}function J(e){return u(d(e).msCrypto)}function V(){return F((function(e,t){d(e).randomFillSync(f(t))}),arguments)}function H(){return F((function(e,t){d(e).getRandomValues(d(t))}),arguments)}function Y(e,t){return u(new Function(a(e,t)))}function ee(){return F((function(e,t){return u(d(e).call(d(t)))}),arguments)}function te(e){return u(d(e))}function ne(){return F((function(){return u(self.self)}),arguments)}function oe(){return F((function(){return u(window.window)}),arguments)}function _e(){return F((function(){return u(globalThis.globalThis)}),arguments)}function re(){return F((function(){return u(global.global)}),arguments)}function ie(e){return void 0===d(e)}function se(){return F((function(e,t,n){return u(d(e).call(d(t),d(n)))}),arguments)}function ae(e){return u(d(e).buffer)}function le(e,t,n){return u(new Uint8Array(d(e),t>>>0,n>>>0))}function ce(e){return u(new Uint8Array(d(e)))}function ue(e,t,n){d(e).set(d(t),n>>>0)}function de(e){return u(new Uint8Array(e>>>0))}function fe(e,t,n){return u(d(e).subarray(t>>>0,n>>>0))}function pe(e,t){throw new Error(a(e,t))}function be(){return u(o.memory)}},4521:(e,t,n)=>{n.d(t,{A:()=>a});var o=n(641);const _={role:"presentation",class:"el"},r=(0,o.Lk)("h1",null,"Have a wonderful journey!",-1),i=(0,o.Lk)("p",null," The file has been saved to your Downloads folder ",-1),s={emits:["file-download"]},a=(0,n(6262).A)(s,[["render",function(e,t,n,s,a,l){const c=(0,o.g2)("a-button");return(0,o.uX)(),(0,o.CE)("div",_,[r,i,(0,o.bF)(c,{type:"primary",onClick:t[0]||(t[0]=t=>e.$emit("file-download"))},{default:(0,o.k6)((()=>[(0,o.eW)("Save Another Copy")])),_:1})])}]])},6461:(e,t,n)=>{n.d(t,{A:()=>a});var o=n(641);const _={role:"presentation",class:"el"},r=(0,o.Lk)("h1",null,"Select the game file",-1),i=(0,o.Lk)("div",{class:"uploader-hint"},[(0,o.Lk)("p",{class:"ant-upload-text"},"Click this or drag file (.sfc, .smc) to this area"),(0,o.Lk)("p",{class:"ant-upload-hint"}," This application runs entirely in your browser. No files are uploaded to any server ")],-1),s={data:()=>({fileList:[]}),emits:["file-selected"],methods:{async fileSelected(e){let t=await function(e){return new Promise(((t,n)=>{const o=new FileReader;o.onload=()=>{const e=new Uint8Array(o.result);t(e)},o.onerror=n,o.readAsArrayBuffer(e)}))}(e);return this.$emit("file-selected",{bytes:t,name:e.name||"download.sfc"}),!1}}},a=(0,n(6262).A)(s,[["render",function(e,t,n,s,a,l){const c=(0,o.g2)("a-upload-dragger");return(0,o.uX)(),(0,o.CE)("div",_,[r,(0,o.bF)(c,{name:"file","before-upload":l.fileSelected},{default:(0,o.k6)((()=>[i])),_:1},8,["before-upload"])])}]])},2062:(e,t,n)=>{n.d(t,{A:()=>u});var o=n(641),_=n(33);const r={key:0,class:"archipelago"},i=[(0,o.Lk)("img",{src:"https://archipelago.gg/static/static/branding/header-logo.svg"},null,-1)],s={key:1,class:"alttpr"},a=[(0,o.Lk)("img",{src:"https://alttpr.com/i/logo.png"},null,-1)],l={key:2},c={props:["game_type"]},u=(0,n(6262).A)(c,[["render",function(e,t,n,c,u,d){return"Archipelago"==n.game_type?((0,o.uX)(),(0,o.CE)("div",r,i)):"A Link to the Past Randomizer"==n.game_type?((0,o.uX)(),(0,o.CE)("div",s,a)):((0,o.uX)(),(0,o.CE)("div",l,(0,_.v_)(n.game_type),1))}]])},1596:(e,t,n)=>{n.a(e,(async(e,o)=>{try{n.d(t,{A:()=>a});var _=n(698),r=n(8142),i=n(6262),s=e([r]);r=(s.then?(await s)():s)[0];const a=(0,i.A)(r.A,[["render",_.X]]);o()}catch(e){o(e)}}))},1213:(e,t,n)=>{n.a(e,(async(e,o)=>{try{n.d(t,{A:()=>u});var _=n(3650),r=e([_]);_=(r.then?(await r)():r)[0];const i="INIT",s="FILE_SELECTED",a="processing",l="DONE",c="ERROR",u={data:()=>({state:i,errorMessage:"",game_type:"",supported:!1,opts:{},input_name:"",input_buffer:void 0,output_buffer:void 0}),mounted(){window.__init_spritzer||(window.__init_spritzer=!0,_.Ts())},methods:{fileSelected({bytes:e,name:t}){const n=_.B1(e);this.input_name=t,this.opts=n.options,this.supported=n.supported,this.game_type=n.game_type,this.input_buffer=e,this.state=s},processZelda3(){try{this.state=a,this.errorMessage="",this.output_buffer=_.Mi(this.input_buffer,this.opts),this.state=l,d(this.output_buffer,this.input_name)}catch(e){this.state=c,this.errorMessage=e.stack}},download(){d(this.output_buffer,this.input_name)}}};function d(e,t){const n=new Blob([e]),o=URL.createObjectURL(n),_=document.createElement("a");_.href=o,_.download=t,_.click(),URL.revokeObjectURL(o),_.remove()}o()}catch(f){o(f)}}))},8142:(e,t,n)=>{n.a(e,(async(e,o)=>{try{n.d(t,{A:()=>_.A});var _=n(1213),r=e([_]);_=(r.then?(await r)():r)[0],o()}catch(e){o(e)}}))},698:(e,t,n)=>{n.d(t,{X:()=>M});var o=n(641),_=n(33);const r={key:0},i={key:1,class:"randomize-option-form"},s=(0,o.Lk)("h1",null,"Options",-1),a={class:"randomizer-option-field",role:"presentation"},l=(0,o.Lk)("label",null,"Item Randomizer",-1),c={class:"randomizer-option-field",role:"presentation"},u=(0,o.Lk)("label",null,"Seed",-1),d={class:"randomizer-option-field",role:"presentation"},f=(0,o.Lk)("label",null,"Overworld Balancing",-1),p={class:"randomizer-option-field",role:"presentation"},b=(0,o.Lk)("label",null,"Overworld Enemy Shuffle",-1),g={class:"randomizer-option-field",role:"presentation"},w=(0,o.Lk)("label",null,"Underworld Balancing",-1),h={class:"randomizer-option-field",role:"presentation"},m=(0,o.Lk)("label",null,"Underworld Enemy Shuffle",-1),y={class:"randomizer-option-field",role:"presentation"},k=(0,o.Lk)("label",null,"Shadow Bees",-1),v={class:"randomizer-option-field",role:"presentation"},L=(0,o.Lk)("label",null,"Boss Shuffle",-1),z={class:"randomizer-option-field",role:"presentation"},A=(0,o.Lk)("label",null,"Killable Thieves",-1),F={class:"randomizer-option-field",role:"presentation"},E=(0,o.Lk)("label",null,"Mushroom Shuffle",-1),S={class:"randomize-option-footer"},T={key:1,class:"unsupported"},C=(0,o.Lk)("h1",null,"Sorry :(",-1),U=(0,o.Lk)("p",null,[(0,o.Lk)("em",null,"This game/combination is not supported, yet")],-1),R=(0,o.Lk)("p",null,[(0,o.eW)("Check "),(0,o.Lk)("a",{href:"/spritzer/guide.html#unsupported-games"},"Unsupported Games"),(0,o.eW)(" in the guide to troubleshoot this. You may need to remove some options from your Item Randomizer")],-1),X={key:2},I=(0,o.Lk)("h1",null,"I AM ERROR",-1),O={class:"language-bash"},x={key:3};function M(e,t,n,M,W,B){const D=(0,o.g2)("a-config-provider"),P=(0,o.g2)("FileSelect"),j=(0,o.g2)("GameTypeImage"),K=(0,o.g2)("a-input"),q=(0,o.g2)("a-select-option"),N=(0,o.g2)("a-select"),$=(0,o.g2)("a-switch"),Q=(0,o.g2)("a-button"),G=(0,o.g2)("Confirmation");return(0,o.uX)(),(0,o.CE)(o.FK,null,[(0,o.bF)(D,{theme:{token:{colorPrimary:"#127900"}}}),"INIT"==W.state?((0,o.uX)(),(0,o.CE)("div",r,[(0,o.bF)(P,{onFileSelected:t[0]||(t[0]=e=>B.fileSelected(e))})])):(0,o.Q3)("",!0),"FILE_SELECTED"==W.state?((0,o.uX)(),(0,o.CE)("div",i,[W.supported?((0,o.uX)(),(0,o.CE)(o.FK,{key:0},[s,(0,o.Lk)("div",a,[l,(0,o.bF)(j,{game_type:W.game_type},null,8,["game_type"])]),(0,o.Lk)("div",c,[u,(0,o.bF)(K,{value:W.opts.seed,"onUpdate:value":t[1]||(t[1]=e=>W.opts.seed=e),addonBefore:"⋆",placeholder:"Please enter a random string"},null,8,["value"])]),(0,o.Lk)("div",d,[f,(0,o.bF)(N,{ref:"select",value:W.opts.ow_balancing,"onUpdate:value":t[2]||(t[2]=e=>W.opts.ow_balancing=e)},{default:(0,o.k6)((()=>[((0,o.uX)(!0),(0,o.CE)(o.FK,null,(0,o.pI)(W.opts.balancing_options,(e=>((0,o.uX)(),(0,o.Wv)(q,{value:e},{default:(0,o.k6)((()=>[(0,o.eW)((0,_.v_)(e),1)])),_:2},1032,["value"])))),256))])),_:1},8,["value"])]),(0,o.Lk)("div",p,[b,(0,o.bF)(N,{ref:"select",value:W.opts.ow_enemy_shuffle,"onUpdate:value":t[3]||(t[3]=e=>W.opts.ow_enemy_shuffle=e)},{default:(0,o.k6)((()=>[((0,o.uX)(!0),(0,o.CE)(o.FK,null,(0,o.pI)(W.opts.ow_enemy_shuffle_options,(e=>((0,o.uX)(),(0,o.Wv)(q,{value:e},{default:(0,o.k6)((()=>[(0,o.eW)((0,_.v_)(e),1)])),_:2},1032,["value"])))),256))])),_:1},8,["value"])]),(0,o.Lk)("div",g,[w,(0,o.bF)(N,{ref:"select",value:W.opts.uw_balancing,"onUpdate:value":t[4]||(t[4]=e=>W.opts.uw_balancing=e)},{default:(0,o.k6)((()=>[((0,o.uX)(!0),(0,o.CE)(o.FK,null,(0,o.pI)(W.opts.balancing_options,(e=>((0,o.uX)(),(0,o.Wv)(q,{value:e},{default:(0,o.k6)((()=>[(0,o.eW)((0,_.v_)(e),1)])),_:2},1032,["value"])))),256))])),_:1},8,["value"])]),(0,o.Lk)("div",h,[m,(0,o.bF)(N,{ref:"select",value:W.opts.uw_enemy_shuffle,"onUpdate:value":t[5]||(t[5]=e=>W.opts.uw_enemy_shuffle=e)},{default:(0,o.k6)((()=>[((0,o.uX)(!0),(0,o.CE)(o.FK,null,(0,o.pI)(W.opts.ow_enemy_shuffle_options,(e=>((0,o.uX)(),(0,o.Wv)(q,{value:e},{default:(0,o.k6)((()=>[(0,o.eW)((0,_.v_)(e),1)])),_:2},1032,["value"])))),256))])),_:1},8,["value"])]),(0,o.Lk)("div",y,[k,(0,o.Lk)("div",null,[(0,o.bF)($,{checked:W.opts.shadow_bees,"onUpdate:checked":t[6]||(t[6]=e=>W.opts.shadow_bees=e)},null,8,["checked"])])]),(0,o.Lk)("div",v,[L,(0,o.Lk)("div",null,[(0,o.bF)($,{checked:W.opts.boss_shuffle,"onUpdate:checked":t[7]||(t[7]=e=>W.opts.boss_shuffle=e)},null,8,["checked"])])]),(0,o.Lk)("div",z,[A,(0,o.Lk)("div",null,[(0,o.bF)($,{checked:W.opts.killable_thieves,"onUpdate:checked":t[8]||(t[8]=e=>W.opts.killable_thieves=e)},null,8,["checked"])])]),(0,o.Lk)("div",F,[E,(0,o.Lk)("div",null,[(0,o.bF)($,{checked:W.opts.mushroom_shuffle,"onUpdate:checked":t[9]||(t[9]=e=>W.opts.mushroom_shuffle=e)},null,8,["checked"])])]),(0,o.Lk)("div",S,[(0,o.bF)(Q,{type:"primary",shape:"round",size:"large",onClick:t[10]||(t[10]=e=>B.processZelda3())},{default:(0,o.k6)((()=>[(0,o.eW)("Roll Game")])),_:1})])],64)):((0,o.uX)(),(0,o.CE)("div",T,[C,(0,o.bF)(j,{game_type:W.game_type},null,8,["game_type"]),U,R]))])):(0,o.Q3)("",!0),"ERROR"==W.state?((0,o.uX)(),(0,o.CE)("div",X,[I,(0,o.Lk)("pre",O,[(0,o.Lk)("code",null,(0,_.v_)(W.errorMessage),1)]),(0,o.bF)(Q,{type:"primary",onClick:t[11]||(t[11]=e=>B.processZelda3())},{default:(0,o.k6)((()=>[(0,o.eW)("Retry")])),_:1})])):(0,o.Q3)("",!0),"DONE"==W.state?((0,o.uX)(),(0,o.CE)("div",x,[(0,o.bF)(G,{onFileDownload:t[12]||(t[12]=e=>B.download())})])):(0,o.Q3)("",!0)],64)}},9063:(e,t,n)=>{n.a(e,(async(e,o)=>{try{n.d(t,{B:()=>p});var _=n(6358),r=n(355),i=n(3825),s=n(3450),a=n(5436),l=n(1599),c=n(5496),u=n(2313),d=n(3173),f=e([d]);d=(f.then?(await f)():f)[0];const p=[_.A,r.A,i.A,s.A,a.A,l.A,c.A,u.A,d.A];o()}catch(e){o(e)}}))},6159:(e,t,n)=>{n.d(t,{J:()=>_,c:()=>o});const o=JSON.parse("{}"),_=Object.fromEntries([["/",{loader:()=>n.e(470).then(n.bind(n,7730)),meta:{title:"Home"}}],["/guide.html",{loader:()=>n.e(94).then(n.bind(n,9550)),meta:{title:"Guide"}}],["/randomize.html",{loader:()=>n.e(985).then(n.bind(n,2503)),meta:{title:"Randomize!"}}],["/404.html",{loader:()=>n.e(490).then(n.bind(n,644)),meta:{title:""}}]])},9354:(e,t,n)=>{n.d(t,{U:()=>o});const o=JSON.parse('{"base":"/spritzer/","lang":"en-US","title":"Spritzer","description":"ALTTP Sprite + Dungeon Randomizer","head":[["link",{"rel":"icon","href":"/spritzer/favicon.ico"}],["link",{"rel":"stylesheet","href":"https://unpkg.com/splitting/dist/splitting.css"}],["link",{"rel":"stylesheet","href":"/spritzer/base.css"}],["script",{"src":"https://unpkg.com/splitting/dist/splitting.min.js"}]],"locales":{}}')},8120:(e,t,n)=>{n.d(t,{K:()=>o});const o=JSON.parse('{"colorMode":"light","colorModeSwitch":false,"contributors":false,"editLink":false,"lastUpdated":false,"logo":"/images/logo.png","repo":"notoriousb1t/spritzer","sidebar":false,"navbar":["/guide","/randomize"],"locales":{"/":{"selectLanguageName":"English"}},"selectLanguageText":"Languages","selectLanguageAriaLabel":"Select language","sidebarDepth":2,"editLinkText":"Edit this page","lastUpdatedText":"Last Updated","contributorsText":"Contributors","notFound":["There\'s nothing here.","How did we get here?","That\'s a Four-Oh-Four.","Looks like we\'ve got some broken links."],"backToHome":"Take me home","openInNewWindow":"open in new window","toggleColorMode":"toggle color mode","toggleSidebar":"toggle sidebar"}')},3173:(e,t,n)=>{n.a(e,(async(e,o)=>{try{n.d(t,{A:()=>g});var _=n(8278),r=n(5220),i=n(641),s=n(3659),a=n(8006),l=n(4521),c=n(6461),u=n(2062),d=n(1596),f=e([d]);function p(){new Promise((e=>{setTimeout(e,450)})).then((()=>{window.requestAnimationFrame((()=>{document.querySelectorAll("h1,h2,h3,h4,h5,h6").forEach(b)}))}))}function b(e){e.classList.contains("splitting")||(e.classList.add("sheika-text"),window.Splitting&&window.Splitting({target:e,by:"chars"}))}d=(f.then?(await f)():f)[0];const g=(0,_.re)({enhance({app:e}){s.A.colorPrimary="black",e.use(a.Ay),e.component("Confirmation",l.A),e.component("FileSelect",c.A),e.component("GameTypeImage",u.A),e.component("RandomizerForm",d.A)},setup(){(0,i.sV)((()=>{(0,r.rd)().afterEach(p),p()}))}});o()}catch(w){o(w)}}))},2554:(e,t,n)=>{var o=n(7070);e.exports=n.v(t,e.id,"4510b2080eb757ba1b89",{"./index_bg.js":{__wbindgen_string_new:o.yc,__wbindgen_object_drop_ref:o.bk,__wbg_debug_7d879afce6cf56cb:o.FT,__wbg_error_8e3928cfb8a43e2b:o.mS,__wbg_error_696630710900ec44:o._J,__wbg_info_80803d9a3f0aad16:o.a0,__wbg_log_151eb4333ef0fe39:o.uI,__wbg_warn_5d3f783b0bae8943:o.f1,__wbg_new_abda76e883ba8a5f:o.V5,__wbg_stack_658279fe44541cf6:o.u$,__wbg_error_f851667af71bcfc6:o.Xu,__wbg_crypto_d05b68a3572bb8ca:o.$D,__wbindgen_is_object:o.qv,__wbg_process_b02b3570280d0366:o.Mx,__wbg_versions_c1cb42213cedf0f5:o.Ke,__wbg_node_43b1089f407e4ec2:o.MF,__wbindgen_is_string:o.Gu,__wbg_require_9a7e0f667ead4995:o.O6,__wbindgen_is_function:o.PR,__wbg_msCrypto_10fc94afee92bd76:o.Si,__wbg_randomFillSync_b70ccbdf4926a99d:o.zQ,__wbg_getRandomValues_7e42b4fb8779dc6d:o.Uv,__wbg_newnoargs_e258087cd0daa0ea:o.Pf,__wbg_call_27c0f87801dedf93:o.cq,__wbindgen_object_clone_ref:o.BZ,__wbg_self_ce0dbfc45cf2f5be:o.cX,__wbg_window_c6fb939a7f436783:o.kh,__wbg_globalThis_d1e6af4856ba331b:o.Kc,__wbg_global_207b558942527489:o.vA,__wbindgen_is_undefined:o.vU,__wbg_call_b3ca7c6051f9bec1:o._m,__wbg_buffer_12d079cc21e14bdb:o.Fm,__wbg_newwithbyteoffsetandlength_aa4a17c33a06e5cb:o.Lo,__wbg_new_63b92bc8671ed464:o.$Z,__wbg_set_a47bac70306a19a7:o.Wv,__wbg_newwithlength_e9b4878cebadb3d3:o.v6,__wbg_subarray_a1f73cd4b5b42fe1:o.if,__wbindgen_throw:o.Qn,__wbindgen_memory:o.Py}})}},e=>{e.O(0,[173,686],(()=>(8731,e(e.s=8731)))),e.O()}]);