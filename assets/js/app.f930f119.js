"use strict";(self.webpackChunkweb=self.webpackChunkweb||[]).push([[524],{3650:(e,t,n)=>{n.a(e,(async(e,o)=>{try{n.d(t,{B1:()=>_.B1,Mi:()=>_.Mi,Ts:()=>_.Ts});var r=n(2554),_=n(7070),i=e([r]);r=(i.then?(await i)():i)[0],(0,_.lI)(r),o()}catch(e){o(e)}}))},7070:(e,t,n)=>{let o;function r(e){o=e}n.d(t,{$D:()=>N,B1:()=>y,BZ:()=>te,Dq:()=>I,Gu:()=>$,I:()=>de,JN:()=>D,Jj:()=>_e,Ke:()=>Q,MF:()=>K,Mi:()=>m,Mx:()=>q,O6:()=>G,PR:()=>V,Py:()=>ge,Qn:()=>pe,Si:()=>J,TQ:()=>ne,Ts:()=>p,Tu:()=>O,Uv:()=>H,V5:()=>j,Xu:()=>B,az:()=>oe,bk:()=>U,eW:()=>ae,gm:()=>ue,hf:()=>X,i$:()=>Y,jI:()=>x,lI:()=>r,lU:()=>M,nA:()=>le,oD:()=>re,p2:()=>se,qv:()=>P,s2:()=>fe,te:()=>ce,u$:()=>W,vR:()=>ee,vU:()=>ie,yc:()=>R,zQ:()=>Z}),e=n.hmd(e);let _=new("undefined"==typeof TextDecoder?(0,e.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});_.decode();let i=null;function s(){return null!==i&&0!==i.byteLength||(i=new Uint8Array(o.memory.buffer)),i}function a(e,t){return e>>>=0,_.decode(s().subarray(e,e+t))}const l=new Array(128).fill(void 0);l.push(void 0,null,!0,!1);let u=l.length;function c(e){u===l.length&&l.push(l.length+1);const t=u;return u=l[t],l[t]=e,t}function d(e){return l[e]}function f(e){const t=d(e);return function(e){e<132||(l[e]=u,u=e)}(e),t}function p(){o.init()}let g=0;function b(e,t){const n=t(1*e.length,1)>>>0;return s().set(e,n/1),g=e.length,n}let w=null;function h(){return null!==w&&0!==w.byteLength||(w=new Int32Array(o.memory.buffer)),w}function m(e,t){try{const u=o.__wbindgen_add_to_stack_pointer(-16),c=b(e,o.__wbindgen_malloc),d=g;!function(e,t){if(!(e instanceof t))throw new Error(`expected instance of ${t.name}`);e.ptr}(t,C);var n=t.__destroy_into_raw();o.process_zelda3(u,c,d,n);var r=h()[u/4+0],_=h()[u/4+1],i=(a=r,l=_,a>>>=0,s().subarray(a/1,a/1+l)).slice();return o.__wbindgen_free(r,1*_,1),i}finally{o.__wbindgen_add_to_stack_pointer(16)}var a,l}function y(e){const t=b(e,o.__wbindgen_malloc),n=g,r=o.detect_options(t,n);return T.__wrap(r)}let k=new("undefined"==typeof TextEncoder?(0,e.require)("util").TextEncoder:TextEncoder)("utf-8");const v="function"==typeof k.encodeInto?function(e,t){return k.encodeInto(e,t)}:function(e,t){const n=k.encode(e);return t.set(n),{read:e.length,written:n.length}};function L(e,t,n){if(void 0===n){const n=k.encode(e),o=t(n.length,1)>>>0;return s().subarray(o,o+n.length).set(n),g=n.length,o}let o=e.length,r=t(o,1)>>>0;const _=s();let i=0;for(;i<o;i++){const t=e.charCodeAt(i);if(t>127)break;_[r+i]=t}if(i!==o){0!==i&&(e=e.slice(i)),r=n(r,o,o=i+3*e.length,1)>>>0;const t=s().subarray(r+i,r+o);i+=v(e,t).written,r=n(r,o,i,1)>>>0}return g=i,r}let z=null;function A(e,t){e>>>=0;const n=(null!==z&&0!==z.byteLength||(z=new Uint32Array(o.memory.buffer)),z).subarray(e/4,e/4+t),r=[];for(let e=0;e<n.length;e++)r.push(f(n[e]));return r}function F(e,t){try{return e.apply(this,t)}catch(e){o.__wbindgen_exn_store(c(e))}}const E="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry((e=>o.__wbg_detectoptionsresult_free(e>>>0)));class T{static __wrap(e){e>>>=0;const t=Object.create(T.prototype);return t.__wbg_ptr=e,E.register(t,t.__wbg_ptr,t),t}__destroy_into_raw(){const e=this.__wbg_ptr;return this.__wbg_ptr=0,E.unregister(this),e}free(){const e=this.__destroy_into_raw();o.__wbg_detectoptionsresult_free(e)}get supported(){return 0!==o.__wbg_get_detectoptionsresult_supported(this.__wbg_ptr)}set supported(e){o.__wbg_set_detectoptionsresult_supported(this.__wbg_ptr,e)}get game_type(){let e,t;try{const _=o.__wbindgen_add_to_stack_pointer(-16);o.detectoptionsresult_game_type(_,this.__wbg_ptr);var n=h()[_/4+0],r=h()[_/4+1];return e=n,t=r,a(n,r)}finally{o.__wbindgen_add_to_stack_pointer(16),o.__wbindgen_free(e,t,1)}}get options(){const e=o.detectoptionsresult_options(this.__wbg_ptr);return C.__wrap(e)}}const S="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry((e=>o.__wbg_z3wasmoptions_free(e>>>0)));class C{static __wrap(e){e>>>=0;const t=Object.create(C.prototype);return t.__wbg_ptr=e,S.register(t,t.__wbg_ptr,t),t}__destroy_into_raw(){const e=this.__wbg_ptr;return this.__wbg_ptr=0,S.unregister(this),e}free(){const e=this.__destroy_into_raw();o.__wbg_z3wasmoptions_free(e)}get boss_shuffle(){return 0!==o.__wbg_get_z3wasmoptions_boss_shuffle(this.__wbg_ptr)}set boss_shuffle(e){o.__wbg_set_z3wasmoptions_boss_shuffle(this.__wbg_ptr,e)}get mushroom_shuffle(){return 0!==o.__wbg_get_z3wasmoptions_mushroom_shuffle(this.__wbg_ptr)}set mushroom_shuffle(e){o.__wbg_set_z3wasmoptions_mushroom_shuffle(this.__wbg_ptr,e)}get shadow_bees(){return 0!==o.__wbg_get_z3wasmoptions_shadow_bees(this.__wbg_ptr)}set shadow_bees(e){o.__wbg_set_z3wasmoptions_shadow_bees(this.__wbg_ptr,e)}get seed(){let e,t;try{const _=o.__wbindgen_add_to_stack_pointer(-16);o.detectoptionsresult_game_type(_,this.__wbg_ptr);var n=h()[_/4+0],r=h()[_/4+1];return e=n,t=r,a(n,r)}finally{o.__wbindgen_add_to_stack_pointer(16),o.__wbindgen_free(e,t,1)}}set seed(e){const t=L(e,o.__wbindgen_malloc,o.__wbindgen_realloc),n=g;o.z3wasmoptions_set_seed(this.__wbg_ptr,t,n)}get ow_balancing(){let e,t;try{const _=o.__wbindgen_add_to_stack_pointer(-16);o.z3wasmoptions_ow_balancing(_,this.__wbg_ptr);var n=h()[_/4+0],r=h()[_/4+1];return e=n,t=r,a(n,r)}finally{o.__wbindgen_add_to_stack_pointer(16),o.__wbindgen_free(e,t,1)}}get ow_enemy_shuffle(){let e,t;try{const _=o.__wbindgen_add_to_stack_pointer(-16);o.z3wasmoptions_ow_enemy_shuffle(_,this.__wbg_ptr);var n=h()[_/4+0],r=h()[_/4+1];return e=n,t=r,a(n,r)}finally{o.__wbindgen_add_to_stack_pointer(16),o.__wbindgen_free(e,t,1)}}set ow_enemy_shuffle(e){const t=L(e,o.__wbindgen_malloc,o.__wbindgen_realloc),n=g;o.z3wasmoptions_set_ow_enemy_shuffle(this.__wbg_ptr,t,n)}set ow_balancing(e){const t=L(e,o.__wbindgen_malloc,o.__wbindgen_realloc),n=g;o.z3wasmoptions_set_ow_balancing(this.__wbg_ptr,t,n)}get uw_enemy_shuffle(){let e,t;try{const _=o.__wbindgen_add_to_stack_pointer(-16);o.z3wasmoptions_uw_enemy_shuffle(_,this.__wbg_ptr);var n=h()[_/4+0],r=h()[_/4+1];return e=n,t=r,a(n,r)}finally{o.__wbindgen_add_to_stack_pointer(16),o.__wbindgen_free(e,t,1)}}set uw_enemy_shuffle(e){const t=L(e,o.__wbindgen_malloc,o.__wbindgen_realloc),n=g;o.z3wasmoptions_set_uw_enemy_shuffle(this.__wbg_ptr,t,n)}get uw_balancing(){let e,t;try{const _=o.__wbindgen_add_to_stack_pointer(-16);o.z3wasmoptions_uw_balancing(_,this.__wbg_ptr);var n=h()[_/4+0],r=h()[_/4+1];return e=n,t=r,a(n,r)}finally{o.__wbindgen_add_to_stack_pointer(16),o.__wbindgen_free(e,t,1)}}set uw_balancing(e){const t=L(e,o.__wbindgen_malloc,o.__wbindgen_realloc),n=g;o.z3wasmoptions_set_uw_balancing(this.__wbg_ptr,t,n)}get balancing_options(){try{const r=o.__wbindgen_add_to_stack_pointer(-16);o.z3wasmoptions_balancing_options(r,this.__wbg_ptr);var e=h()[r/4+0],t=h()[r/4+1],n=A(e,t).slice();return o.__wbindgen_free(e,4*t,4),n}finally{o.__wbindgen_add_to_stack_pointer(16)}}get ow_enemy_shuffle_options(){try{const r=o.__wbindgen_add_to_stack_pointer(-16);o.z3wasmoptions_ow_enemy_shuffle_options(r,this.__wbg_ptr);var e=h()[r/4+0],t=h()[r/4+1],n=A(e,t).slice();return o.__wbindgen_free(e,4*t,4),n}finally{o.__wbindgen_add_to_stack_pointer(16)}}get uw_enemy_shuffle_options(){try{const r=o.__wbindgen_add_to_stack_pointer(-16);o.z3wasmoptions_uw_enemy_shuffle_options(r,this.__wbg_ptr);var e=h()[r/4+0],t=h()[r/4+1],n=A(e,t).slice();return o.__wbindgen_free(e,4*t,4),n}finally{o.__wbindgen_add_to_stack_pointer(16)}}}function R(e,t){return c(a(e,t))}function U(e){f(e)}function I(e,t,n,o){console.debug(d(e),d(t),d(n),d(o))}function X(e){console.error(d(e))}function O(e,t,n,o){console.error(d(e),d(t),d(n),d(o))}function x(e,t,n,o){console.info(d(e),d(t),d(n),d(o))}function M(e,t,n,o){console.log(d(e),d(t),d(n),d(o))}function D(e,t,n,o){console.warn(d(e),d(t),d(n),d(o))}function j(){return c(new Error)}function W(e,t){const n=L(d(t).stack,o.__wbindgen_malloc,o.__wbindgen_realloc),r=g;h()[e/4+1]=r,h()[e/4+0]=n}function B(e,t){let n,r;try{n=e,r=t,console.error(a(e,t))}finally{o.__wbindgen_free(n,r,1)}}function N(e){return c(d(e).crypto)}function P(e){const t=d(e);return"object"==typeof t&&null!==t}function q(e){return c(d(e).process)}function Q(e){return c(d(e).versions)}function K(e){return c(d(e).node)}function $(e){return"string"==typeof d(e)}function J(e){return c(d(e).msCrypto)}function G(){return F((function(){return c(e.require)}),arguments)}function V(e){return"function"==typeof d(e)}function Z(){return F((function(e,t){d(e).randomFillSync(f(t))}),arguments)}function H(){return F((function(e,t){d(e).getRandomValues(d(t))}),arguments)}function Y(e,t){return c(new Function(a(e,t)))}function ee(){return F((function(e,t){return c(d(e).call(d(t)))}),arguments)}function te(e){return c(d(e))}function ne(){return F((function(){return c(self.self)}),arguments)}function oe(){return F((function(){return c(window.window)}),arguments)}function re(){return F((function(){return c(globalThis.globalThis)}),arguments)}function _e(){return F((function(){return c(global.global)}),arguments)}function ie(e){return void 0===d(e)}function se(){return F((function(e,t,n){return c(d(e).call(d(t),d(n)))}),arguments)}function ae(e){return c(d(e).buffer)}function le(e,t,n){return c(new Uint8Array(d(e),t>>>0,n>>>0))}function ue(e){return c(new Uint8Array(d(e)))}function ce(e,t,n){d(e).set(d(t),n>>>0)}function de(e){return c(new Uint8Array(e>>>0))}function fe(e,t,n){return c(d(e).subarray(t>>>0,n>>>0))}function pe(e,t){throw new Error(a(e,t))}function ge(){return c(o.memory)}},4521:(e,t,n)=>{n.d(t,{A:()=>a});var o=n(641);const r={role:"presentation",class:"el"},_=(0,o.Lk)("h1",null,"Have a wonderful journey!",-1),i=(0,o.Lk)("p",null," The file has been saved to your Downloads folder ",-1),s={emits:["file-download"]},a=(0,n(6262).A)(s,[["render",function(e,t,n,s,a,l){const u=(0,o.g2)("a-button");return(0,o.uX)(),(0,o.CE)("div",r,[_,i,(0,o.bF)(u,{type:"primary",onClick:t[0]||(t[0]=t=>e.$emit("file-download"))},{default:(0,o.k6)((()=>[(0,o.eW)("Save Another Copy")])),_:1})])}]])},6461:(e,t,n)=>{n.d(t,{A:()=>a});var o=n(641);const r={role:"presentation",class:"el"},_=(0,o.Lk)("h1",null,"Select the game file",-1),i=(0,o.Lk)("div",{class:"uploader-hint"},[(0,o.Lk)("p",{class:"ant-upload-text"},"Click this or drag file (.sfc, .smc) to this area"),(0,o.Lk)("p",{class:"ant-upload-hint"}," This application runs entirely in your browser. No files are uploaded to any server ")],-1),s={data:()=>({fileList:[]}),emits:["file-selected"],methods:{async fileSelected(e){let t=await function(e){return new Promise(((t,n)=>{const o=new FileReader;o.onload=()=>{const e=new Uint8Array(o.result);t(e)},o.onerror=n,o.readAsArrayBuffer(e)}))}(e);return this.$emit("file-selected",{bytes:t,name:e.name||"download.sfc"}),!1}}},a=(0,n(6262).A)(s,[["render",function(e,t,n,s,a,l){const u=(0,o.g2)("a-upload-dragger");return(0,o.uX)(),(0,o.CE)("div",r,[_,(0,o.bF)(u,{name:"file","before-upload":l.fileSelected},{default:(0,o.k6)((()=>[i])),_:1},8,["before-upload"])])}]])},2062:(e,t,n)=>{n.d(t,{A:()=>c});var o=n(641),r=n(33);const _={key:0,class:"archipelago"},i=[(0,o.Lk)("img",{src:"https://archipelago.gg/static/static/branding/header-logo.svg"},null,-1)],s={key:1,class:"alttpr"},a=[(0,o.Lk)("img",{src:"https://alttpr.com/i/logo.png"},null,-1)],l={key:2},u={props:["game_type"]},c=(0,n(6262).A)(u,[["render",function(e,t,n,u,c,d){return"Archipelago"==n.game_type?((0,o.uX)(),(0,o.CE)("div",_,i)):"A Link to the Past Randomizer"==n.game_type?((0,o.uX)(),(0,o.CE)("div",s,a)):((0,o.uX)(),(0,o.CE)("div",l,(0,r.v_)(n.game_type),1))}]])},1596:(e,t,n)=>{n.a(e,(async(e,o)=>{try{n.d(t,{A:()=>a});var r=n(1472),_=n(8142),i=n(6262),s=e([_]);_=(s.then?(await s)():s)[0];const a=(0,i.A)(_.A,[["render",r.X]]);o()}catch(e){o(e)}}))},1213:(e,t,n)=>{n.a(e,(async(e,o)=>{try{n.d(t,{A:()=>c});var r=n(3650),_=e([r]);r=(_.then?(await _)():_)[0];const i="INIT",s="FILE_SELECTED",a="processing",l="DONE",u="ERROR",c={data:()=>({state:i,errorMessage:"",game_type:"",supported:!1,opts:{},input_name:"",input_buffer:void 0,output_buffer:void 0}),mounted(){window.__init_spritzer||(window.__init_spritzer=!0,r.Ts())},methods:{fileSelected({bytes:e,name:t}){const n=r.B1(e);this.input_name=t,this.opts=n.options,this.supported=n.supported,this.game_type=n.game_type,this.input_buffer=e,this.state=s},processZelda3(){try{this.state=a,this.errorMessage="",this.output_buffer=r.Mi(this.input_buffer,this.opts),this.state=l,d(this.output_buffer,this.input_name)}catch(e){this.state=u,this.errorMessage=e.stack}},download(){d(this.output_buffer,this.input_name)}}};function d(e,t){const n=new Blob([e]),o=URL.createObjectURL(n),r=document.createElement("a");r.href=o,r.download=t,r.click(),URL.revokeObjectURL(o),r.remove()}o()}catch(f){o(f)}}))},8142:(e,t,n)=>{n.a(e,(async(e,o)=>{try{n.d(t,{A:()=>r.A});var r=n(1213),_=e([r]);r=(_.then?(await _)():_)[0],o()}catch(e){o(e)}}))},1472:(e,t,n)=>{n.d(t,{X:()=>O});var o=n(641),r=n(33);const _={key:0},i={key:1,class:"randomize-option-form"},s=(0,o.Lk)("h1",null,"Options",-1),a={class:"randomizer-option-field",role:"presentation"},l=(0,o.Lk)("label",null,"Item Randomizer",-1),u={class:"randomizer-option-field",role:"presentation"},c=(0,o.Lk)("label",null,"Seed",-1),d={class:"randomizer-option-field",role:"presentation"},f=(0,o.Lk)("label",null,"Overworld Balancing",-1),p={class:"randomizer-option-field",role:"presentation"},g=(0,o.Lk)("label",null,"Overworld Enemy Shuffle",-1),b={class:"randomizer-option-field",role:"presentation"},w=(0,o.Lk)("label",null,"Underworld Balancing",-1),h={class:"randomizer-option-field",role:"presentation"},m=(0,o.Lk)("label",null,"Underworld Enemy Shuffle",-1),y={class:"randomizer-option-field",role:"presentation"},k=(0,o.Lk)("label",null,"Shadow Bees",-1),v={class:"randomizer-option-field",role:"presentation"},L=(0,o.Lk)("label",null,"Boss Shuffle",-1),z={class:"randomizer-option-field",role:"presentation"},A=(0,o.Lk)("label",null,"Mushroom Shuffle",-1),F={class:"randomize-option-footer"},E={key:1,class:"unsupported"},T=(0,o.Lk)("h1",null,"Sorry :(",-1),S=(0,o.Lk)("p",null,[(0,o.Lk)("em",null,"This game/combination is not supported, yet")],-1),C=(0,o.Lk)("p",null,[(0,o.eW)("Check "),(0,o.Lk)("a",{href:"/spritzer/guide.html#unsupported-games"},"Unsupported Games"),(0,o.eW)(" in the guide to troubleshoot this. You may need to remove some options from your Item Randomizer")],-1),R={key:2},U=(0,o.Lk)("h1",null,"I AM ERROR",-1),I={class:"language-bash"},X={key:3};function O(e,t,n,O,x,M){const D=(0,o.g2)("a-config-provider"),j=(0,o.g2)("FileSelect"),W=(0,o.g2)("GameTypeImage"),B=(0,o.g2)("a-input"),N=(0,o.g2)("a-select-option"),P=(0,o.g2)("a-select"),q=(0,o.g2)("a-switch"),Q=(0,o.g2)("a-button"),K=(0,o.g2)("Confirmation");return(0,o.uX)(),(0,o.CE)(o.FK,null,[(0,o.bF)(D,{theme:{token:{colorPrimary:"#127900"}}}),"INIT"==x.state?((0,o.uX)(),(0,o.CE)("div",_,[(0,o.bF)(j,{onFileSelected:t[0]||(t[0]=e=>M.fileSelected(e))})])):(0,o.Q3)("",!0),"FILE_SELECTED"==x.state?((0,o.uX)(),(0,o.CE)("div",i,[x.supported?((0,o.uX)(),(0,o.CE)(o.FK,{key:0},[s,(0,o.Lk)("div",a,[l,(0,o.bF)(W,{game_type:x.game_type},null,8,["game_type"])]),(0,o.Lk)("div",u,[c,(0,o.bF)(B,{value:x.opts.seed,"onUpdate:value":t[1]||(t[1]=e=>x.opts.seed=e),addonBefore:"⋆",placeholder:"Please enter a random string"},null,8,["value"])]),(0,o.Lk)("div",d,[f,(0,o.bF)(P,{ref:"select",value:x.opts.ow_balancing,"onUpdate:value":t[2]||(t[2]=e=>x.opts.ow_balancing=e)},{default:(0,o.k6)((()=>[((0,o.uX)(!0),(0,o.CE)(o.FK,null,(0,o.pI)(x.opts.balancing_options,(e=>((0,o.uX)(),(0,o.Wv)(N,{value:e},{default:(0,o.k6)((()=>[(0,o.eW)((0,r.v_)(e),1)])),_:2},1032,["value"])))),256))])),_:1},8,["value"])]),(0,o.Lk)("div",p,[g,(0,o.bF)(P,{ref:"select",value:x.opts.ow_enemy_shuffle,"onUpdate:value":t[3]||(t[3]=e=>x.opts.ow_enemy_shuffle=e)},{default:(0,o.k6)((()=>[((0,o.uX)(!0),(0,o.CE)(o.FK,null,(0,o.pI)(x.opts.ow_enemy_shuffle_options,(e=>((0,o.uX)(),(0,o.Wv)(N,{value:e},{default:(0,o.k6)((()=>[(0,o.eW)((0,r.v_)(e),1)])),_:2},1032,["value"])))),256))])),_:1},8,["value"])]),(0,o.Lk)("div",b,[w,(0,o.bF)(P,{ref:"select",value:x.opts.uw_balancing,"onUpdate:value":t[4]||(t[4]=e=>x.opts.uw_balancing=e)},{default:(0,o.k6)((()=>[((0,o.uX)(!0),(0,o.CE)(o.FK,null,(0,o.pI)(x.opts.balancing_options,(e=>((0,o.uX)(),(0,o.Wv)(N,{value:e},{default:(0,o.k6)((()=>[(0,o.eW)((0,r.v_)(e),1)])),_:2},1032,["value"])))),256))])),_:1},8,["value"])]),(0,o.Lk)("div",h,[m,(0,o.bF)(P,{ref:"select",value:x.opts.uw_enemy_shuffle,"onUpdate:value":t[5]||(t[5]=e=>x.opts.uw_enemy_shuffle=e)},{default:(0,o.k6)((()=>[((0,o.uX)(!0),(0,o.CE)(o.FK,null,(0,o.pI)(x.opts.ow_enemy_shuffle_options,(e=>((0,o.uX)(),(0,o.Wv)(N,{value:e},{default:(0,o.k6)((()=>[(0,o.eW)((0,r.v_)(e),1)])),_:2},1032,["value"])))),256))])),_:1},8,["value"])]),(0,o.Lk)("div",y,[k,(0,o.Lk)("div",null,[(0,o.bF)(q,{checked:x.opts.shadow_bees,"onUpdate:checked":t[6]||(t[6]=e=>x.opts.shadow_bees=e)},null,8,["checked"])])]),(0,o.Lk)("div",v,[L,(0,o.Lk)("div",null,[(0,o.bF)(q,{checked:x.opts.boss_shuffle,"onUpdate:checked":t[7]||(t[7]=e=>x.opts.boss_shuffle=e)},null,8,["checked"])])]),(0,o.Lk)("div",z,[A,(0,o.Lk)("div",null,[(0,o.bF)(q,{checked:x.opts.mushroom_shuffle,"onUpdate:checked":t[8]||(t[8]=e=>x.opts.mushroom_shuffle=e)},null,8,["checked"])])]),(0,o.Lk)("div",F,[(0,o.bF)(Q,{type:"primary",shape:"round",size:"large",onClick:t[9]||(t[9]=e=>M.processZelda3())},{default:(0,o.k6)((()=>[(0,o.eW)("Roll Game")])),_:1})])],64)):((0,o.uX)(),(0,o.CE)("div",E,[T,(0,o.bF)(W,{game_type:x.game_type},null,8,["game_type"]),S,C]))])):(0,o.Q3)("",!0),"ERROR"==x.state?((0,o.uX)(),(0,o.CE)("div",R,[U,(0,o.Lk)("pre",I,[(0,o.Lk)("code",null,(0,r.v_)(x.errorMessage),1)]),(0,o.bF)(Q,{type:"primary",onClick:t[10]||(t[10]=e=>M.processZelda3())},{default:(0,o.k6)((()=>[(0,o.eW)("Retry")])),_:1})])):(0,o.Q3)("",!0),"DONE"==x.state?((0,o.uX)(),(0,o.CE)("div",X,[(0,o.bF)(K,{onFileDownload:t[11]||(t[11]=e=>M.download())})])):(0,o.Q3)("",!0)],64)}},9063:(e,t,n)=>{n.a(e,(async(e,o)=>{try{n.d(t,{B:()=>p});var r=n(6358),_=n(355),i=n(3825),s=n(3450),a=n(5436),l=n(1599),u=n(5496),c=n(2313),d=n(3173),f=e([d]);d=(f.then?(await f)():f)[0];const p=[r.A,_.A,i.A,s.A,a.A,l.A,u.A,c.A,d.A];o()}catch(e){o(e)}}))},6159:(e,t,n)=>{n.d(t,{J:()=>r,c:()=>o});const o=JSON.parse("{}"),r=Object.fromEntries([["/",{loader:()=>n.e(470).then(n.bind(n,7730)),meta:{title:"Home"}}],["/guide.html",{loader:()=>n.e(94).then(n.bind(n,4718)),meta:{title:"Guide"}}],["/randomize.html",{loader:()=>n.e(985).then(n.bind(n,2503)),meta:{title:"Randomize!"}}],["/404.html",{loader:()=>n.e(490).then(n.bind(n,644)),meta:{title:""}}]])},9354:(e,t,n)=>{n.d(t,{U:()=>o});const o=JSON.parse('{"base":"/spritzer/","lang":"en-US","title":"Spritzer","description":"ALTTP Sprite + Dungeon Randomizer","head":[["link",{"rel":"icon","href":"/spritzer/favicon.ico"}],["link",{"rel":"stylesheet","href":"https://unpkg.com/splitting/dist/splitting.css"}],["link",{"rel":"stylesheet","href":"/spritzer/base.css"}],["script",{"src":"https://unpkg.com/splitting/dist/splitting.min.js"}]],"locales":{}}')},8120:(e,t,n)=>{n.d(t,{K:()=>o});const o=JSON.parse('{"colorMode":"light","colorModeSwitch":false,"contributors":false,"editLink":false,"lastUpdated":false,"logo":"/images/logo.png","repo":"notoriousb1t/spritzer","sidebar":false,"navbar":["/guide","/randomize"],"locales":{"/":{"selectLanguageName":"English"}},"selectLanguageText":"Languages","selectLanguageAriaLabel":"Select language","sidebarDepth":2,"editLinkText":"Edit this page","lastUpdatedText":"Last Updated","contributorsText":"Contributors","notFound":["There\'s nothing here.","How did we get here?","That\'s a Four-Oh-Four.","Looks like we\'ve got some broken links."],"backToHome":"Take me home","openInNewWindow":"open in new window","toggleColorMode":"toggle color mode","toggleSidebar":"toggle sidebar"}')},3173:(e,t,n)=>{n.a(e,(async(e,o)=>{try{n.d(t,{A:()=>b});var r=n(8278),_=n(5220),i=n(641),s=n(3659),a=n(8006),l=n(4521),u=n(6461),c=n(2062),d=n(1596),f=e([d]);function p(){new Promise((e=>{setTimeout(e,450)})).then((()=>{window.requestAnimationFrame((()=>{document.querySelectorAll("h1,h2,h3,h4,h5,h6").forEach(g)}))}))}function g(e){e.classList.contains("splitting")||(e.classList.add("sheika-text"),window.Splitting&&window.Splitting({target:e,by:"chars"}))}d=(f.then?(await f)():f)[0];const b=(0,r.re)({enhance({app:e}){s.A.colorPrimary="black",e.use(a.Ay),e.component("Confirmation",l.A),e.component("FileSelect",u.A),e.component("GameTypeImage",c.A),e.component("RandomizerForm",d.A)},setup(){(0,i.sV)((()=>{(0,_.rd)().afterEach(p),p()}))}});o()}catch(w){o(w)}}))},2554:(e,t,n)=>{var o=n(7070);e.exports=n.v(t,e.id,"27ac110d058830e9f722",{"./index_bg.js":{__wbindgen_string_new:o.yc,__wbindgen_object_drop_ref:o.bk,__wbg_debug_a0de8bb323ed3e46:o.Dq,__wbg_error_b834525fe62708f5:o.hf,__wbg_error_621c11091f132493:o.Tu,__wbg_info_0d0cfdd0035dc3f3:o.jI,__wbg_log_c957bb910fba09c1:o.lU,__wbg_warn_0912b6f3e9479355:o.JN,__wbg_new_abda76e883ba8a5f:o.V5,__wbg_stack_658279fe44541cf6:o.u$,__wbg_error_f851667af71bcfc6:o.Xu,__wbg_crypto_d05b68a3572bb8ca:o.$D,__wbindgen_is_object:o.qv,__wbg_process_b02b3570280d0366:o.Mx,__wbg_versions_c1cb42213cedf0f5:o.Ke,__wbg_node_43b1089f407e4ec2:o.MF,__wbindgen_is_string:o.Gu,__wbg_msCrypto_10fc94afee92bd76:o.Si,__wbg_require_9a7e0f667ead4995:o.O6,__wbindgen_is_function:o.PR,__wbg_randomFillSync_b70ccbdf4926a99d:o.zQ,__wbg_getRandomValues_7e42b4fb8779dc6d:o.Uv,__wbg_newnoargs_cfecb3965268594c:o.i$,__wbg_call_3f093dd26d5569f8:o.vR,__wbindgen_object_clone_ref:o.BZ,__wbg_self_05040bd9523805b9:o.TQ,__wbg_window_adc720039f2cb14f:o.az,__wbg_globalThis_622105db80c1457d:o.oD,__wbg_global_f56b013ed9bcf359:o.Jj,__wbindgen_is_undefined:o.vU,__wbg_call_67f2111acd2dfdb6:o.p2,__wbg_buffer_b914fb8b50ebbc3e:o.eW,__wbg_newwithbyteoffsetandlength_0de9ee56e9f6ee6e:o.nA,__wbg_new_b1f2d6842d615181:o.gm,__wbg_set_7d988c98e6ced92d:o.te,__wbg_newwithlength_0d03cef43b68a530:o.I,__wbg_subarray_adc418253d76e2f1:o.s2,__wbindgen_throw:o.Qn,__wbindgen_memory:o.Py}})}},e=>{e.O(0,[118,686],(()=>(8731,e(e.s=8731)))),e.O()}]);