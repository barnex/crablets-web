let _=16,Q=`undefined`,a2=7709,a1=7454,Y=`Object`,O=null,$=4,T=0,R=`utf-8`,P=1,V=`function`,W=`number`,X=`string`,M=Array,S=Error,Z=FinalizationRegistry,a0=Object,U=Uint8Array,N=undefined;var E=((a,b)=>{a=a>>>T;return D().subarray(a/$,a/$+ b)});var z=((b,c)=>{a.wasm_bindgen__convert__closures__invoke0_mut__h7f2d98d254d271bb(b,c)});var p=(a=>a===N||a===O);var w=((b,c,d,e)=>{const f={a:b,b:c,cnt:P,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=T;try{return e(c,f.b,...b)}finally{if(--f.cnt===T){a.__wbindgen_export_2.get(f.dtor)(c,f.b);v.unregister(f)}else{f.a=c}}};g.original=f;v.register(g,f,f);return g});var x=((b,c,d)=>{a.wasm_bindgen__convert__closures__invoke1_mut__h8fcfb372a502c7e6(b,c,g(d))});var t=(()=>{if(s===O||s.byteLength===T){s=new Float64Array(a.memory.buffer)};return s});var e=(a=>{if(a<132)return;b[a]=d;d=a});var j=(()=>{if(i===O||i.byteLength===T){i=new U(a.memory.buffer)};return i});var g=(a=>{if(d===b.length)b.push(b.length+ P);const c=d;d=b[c];b[c]=a;return c});var u=(a=>{const b=typeof a;if(b==W||b==`boolean`||a==O){return `${a}`};if(b==X){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==O){return `Symbol`}else{return `Symbol(${b})`}};if(b==V){const b=a.name;if(typeof b==X&&b.length>T){return `Function(${b})`}else{return `Function`}};if(M.isArray(a)){const b=a.length;let c=`[`;if(b>T){c+=u(a[T])};for(let d=P;d<b;d++){c+=`, `+ u(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>P){d=c[P]}else{return toString.call(a)};if(d==Y){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return Y}};if(a instanceof S){return `${a.name}: ${a.message}\n${a.stack}`};return d});var L=(async(b)=>{if(a!==N)return a;if(typeof b===Q){b=new URL(`app_bg.wasm`,import.meta.url)};const c=H();if(typeof b===X||typeof Request===V&&b instanceof Request||typeof URL===V&&b instanceof URL){b=fetch(b)};I(c);const {instance:d,module:e}=await G(await b,c);return J(d,e)});function F(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_exn_store(g(b))}}var A=((b,c,d)=>{a.wasm_bindgen__convert__closures__invoke1_mut__h25017c89f2dc8269(b,c,g(d))});var K=(b=>{if(a!==N)return a;const c=H();I(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return J(d,b)});var c=(a=>b[a]);var H=(()=>{const b={};b.wbg={};b.wbg.__wbindgen_object_drop_ref=(a=>{f(a)});b.wbg.__wbindgen_object_clone_ref=(a=>{const b=c(a);return g(b)});b.wbg.__wbindgen_string_new=((a,b)=>{const c=k(a,b);return g(c)});b.wbg.__wbindgen_cb_drop=(a=>{const b=f(a).original;if(b.cnt--==P){b.a=T;return !0};const c=!1;return c});b.wbg.__wbindgen_is_undefined=(a=>{const b=c(a)===N;return b});b.wbg.__wbindgen_string_get=((b,d)=>{const e=c(d);const f=typeof e===X?e:N;var g=p(f)?T:o(f,a.__wbindgen_malloc,a.__wbindgen_realloc);var h=l;r()[b/$+ P]=h;r()[b/$+ T]=g});b.wbg.__wbg_trace_84825870b6476631=((a,b)=>{console.trace(k(a,b))});b.wbg.__wbg_debug_5f53c9f0b7ddab02=((a,b)=>{console.debug(k(a,b))});b.wbg.__wbg_info_3c29928c828b3708=((a,b)=>{console.info(k(a,b))});b.wbg.__wbg_warn_5cafa3521c57b544=((a,b)=>{console.warn(k(a,b))});b.wbg.__wbindgen_number_get=((a,b)=>{const d=c(b);const e=typeof d===W?d:N;t()[a/8+ P]=p(e)?T:e;r()[a/$+ T]=!p(e)});b.wbg.__wbg_error_f1122c186eb50a3e=((b,c)=>{let d;let e;try{d=b;e=c;console.error(k(b,c))}finally{a.__wbindgen_free(d,e,P)}});b.wbg.__wbg_new_ef92b2d19cb9c942=(()=>{const a=new S();return g(a)});b.wbg.__wbg_stack_f228f9c6cc369ed8=((b,d)=>{const e=c(d).stack;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ P]=g;r()[b/$+ T]=f});b.wbg.__wbg_performance_bdf4f1a290fc5c5c=(a=>{const b=c(a).performance;return g(b)});b.wbg.__wbg_now_d87295c25be68e8b=(a=>{const b=c(a).now();return b});b.wbg.__wbg_instanceof_GpuValidationError_3128431f7a0514f4=(a=>{let b;try{b=c(a) instanceof GPUValidationError}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_message_867097f776344069=((b,d)=>{const e=c(d).message;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ P]=g;r()[b/$+ T]=f});b.wbg.__wbg_instanceof_GpuOutOfMemoryError_b37a08bfb7cee038=(a=>{let b;try{b=c(a) instanceof GPUOutOfMemoryError}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_error_7ced2e8034eb1f3f=(a=>{const b=c(a).error;return g(b)});b.wbg.__wbg_has_d655f3a252d0b10a=((a,b,d)=>{const e=c(a).has(k(b,d));return e});b.wbg.__wbg_maxTextureDimension1D_53351b4a7253c324=(a=>{const b=c(a).maxTextureDimension1D;return b});b.wbg.__wbg_maxTextureDimension2D_26995ffa94733f82=(a=>{const b=c(a).maxTextureDimension2D;return b});b.wbg.__wbg_maxTextureDimension3D_8d77c6d768caef58=(a=>{const b=c(a).maxTextureDimension3D;return b});b.wbg.__wbg_maxTextureArrayLayers_cbf7e90284df66c3=(a=>{const b=c(a).maxTextureArrayLayers;return b});b.wbg.__wbg_maxBindGroups_54fa38a646718d85=(a=>{const b=c(a).maxBindGroups;return b});b.wbg.__wbg_maxBindingsPerBindGroup_e8f7a2792b9ac107=(a=>{const b=c(a).maxBindingsPerBindGroup;return b});b.wbg.__wbg_maxDynamicUniformBuffersPerPipelineLayout_7c5942f359a6fb1b=(a=>{const b=c(a).maxDynamicUniformBuffersPerPipelineLayout;return b});b.wbg.__wbg_maxDynamicStorageBuffersPerPipelineLayout_bd22a382d13e6ef5=(a=>{const b=c(a).maxDynamicStorageBuffersPerPipelineLayout;return b});b.wbg.__wbg_maxSampledTexturesPerShaderStage_5704d5ff400bceee=(a=>{const b=c(a).maxSampledTexturesPerShaderStage;return b});b.wbg.__wbg_maxSamplersPerShaderStage_5e8845f07c33913a=(a=>{const b=c(a).maxSamplersPerShaderStage;return b});b.wbg.__wbg_maxStorageBuffersPerShaderStage_18a674788ed5fdad=(a=>{const b=c(a).maxStorageBuffersPerShaderStage;return b});b.wbg.__wbg_maxStorageTexturesPerShaderStage_bfff5cb8d91bcfcc=(a=>{const b=c(a).maxStorageTexturesPerShaderStage;return b});b.wbg.__wbg_maxUniformBuffersPerShaderStage_ef06df9be2943d45=(a=>{const b=c(a).maxUniformBuffersPerShaderStage;return b});b.wbg.__wbg_maxUniformBufferBindingSize_f84670235a7e5df9=(a=>{const b=c(a).maxUniformBufferBindingSize;return b});b.wbg.__wbg_maxStorageBufferBindingSize_9245cd89c719dbf2=(a=>{const b=c(a).maxStorageBufferBindingSize;return b});b.wbg.__wbg_maxVertexBuffers_73da155813feea78=(a=>{const b=c(a).maxVertexBuffers;return b});b.wbg.__wbg_maxBufferSize_7087869d4548c87d=(a=>{const b=c(a).maxBufferSize;return b});b.wbg.__wbg_maxVertexAttributes_3a0ea01143239608=(a=>{const b=c(a).maxVertexAttributes;return b});b.wbg.__wbg_maxVertexBufferArrayStride_d699c03944dd52d9=(a=>{const b=c(a).maxVertexBufferArrayStride;return b});b.wbg.__wbg_minUniformBufferOffsetAlignment_5574ef5e4f6d62da=(a=>{const b=c(a).minUniformBufferOffsetAlignment;return b});b.wbg.__wbg_minStorageBufferOffsetAlignment_a6666e346184b953=(a=>{const b=c(a).minStorageBufferOffsetAlignment;return b});b.wbg.__wbg_maxInterStageShaderComponents_09be6edd346cb8da=(a=>{const b=c(a).maxInterStageShaderComponents;return b});b.wbg.__wbg_maxComputeWorkgroupStorageSize_58415be93e502f25=(a=>{const b=c(a).maxComputeWorkgroupStorageSize;return b});b.wbg.__wbg_maxComputeInvocationsPerWorkgroup_8aa2f0a5861ce5ef=(a=>{const b=c(a).maxComputeInvocationsPerWorkgroup;return b});b.wbg.__wbg_maxComputeWorkgroupSizeX_789174905500f6c7=(a=>{const b=c(a).maxComputeWorkgroupSizeX;return b});b.wbg.__wbg_maxComputeWorkgroupSizeY_926ec1c24c6136da=(a=>{const b=c(a).maxComputeWorkgroupSizeY;return b});b.wbg.__wbg_maxComputeWorkgroupSizeZ_562c888ae9402be1=(a=>{const b=c(a).maxComputeWorkgroupSizeZ;return b});b.wbg.__wbg_maxComputeWorkgroupsPerDimension_07fa50cdca40e120=(a=>{const b=c(a).maxComputeWorkgroupsPerDimension;return b});b.wbg.__wbg_instanceof_GpuAdapter_76bb05881d5f91d1=(a=>{let b;try{b=c(a) instanceof GPUAdapter}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_queue_9f8d8658085c6f43=(a=>{const b=c(a).queue;return g(b)});b.wbg.__wbindgen_is_object=(a=>{const b=c(a);const d=typeof b===`object`&&b!==O;return d});b.wbg.__wbg_instanceof_GpuCanvasContext_05351086956f1883=(a=>{let b;try{b=c(a) instanceof GPUCanvasContext}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_getMappedRange_8229b08f744819c0=((a,b,d)=>{const e=c(a).getMappedRange(b,d);return g(e)});b.wbg.__wbg_Window_a1459b9c171b6eed=(a=>{const b=c(a).Window;return g(b)});b.wbg.__wbg_WorkerGlobalScope_e1b8bcefd2818e94=(a=>{const b=c(a).WorkerGlobalScope;return g(b)});b.wbg.__wbg_gpu_4ac835f782ad971d=(a=>{const b=c(a).gpu;return g(b)});b.wbg.__wbg_requestAdapter_913357b9788f14cd=((a,b)=>{const d=c(a).requestAdapter(c(b));return g(d)});b.wbg.__wbindgen_number_new=(a=>{const b=a;return g(b)});b.wbg.__wbg_requestDevice_baf0b46015a90431=((a,b)=>{const d=c(a).requestDevice(c(b));return g(d)});b.wbg.__wbg_features_7fd6ee02e18d77a4=(a=>{const b=c(a).features;return g(b)});b.wbg.__wbg_limits_7c1e17ce28ddf954=(a=>{const b=c(a).limits;return g(b)});b.wbg.__wbg_getPreferredCanvasFormat_c57006806f2efe1b=(a=>{const b=c(a).getPreferredCanvasFormat();return g(b)});b.wbg.__wbg_configure_8ae8b7e66a9d6189=((a,b)=>{c(a).configure(c(b))});b.wbg.__wbg_getCurrentTexture_26a07297d850dcb1=(a=>{const b=c(a).getCurrentTexture();return g(b)});b.wbg.__wbg_features_01f848ca4efe700b=(a=>{const b=c(a).features;return g(b)});b.wbg.__wbg_limits_cf6e9ab92d696f0c=(a=>{const b=c(a).limits;return g(b)});b.wbg.__wbg_createShaderModule_6851cf2067c2f947=((a,b)=>{const d=c(a).createShaderModule(c(b));return g(d)});b.wbg.__wbg_createBindGroupLayout_6adcd872318d899a=((a,b)=>{const d=c(a).createBindGroupLayout(c(b));return g(d)});b.wbg.__wbg_createBindGroup_5ac37963cb812b24=((a,b)=>{const d=c(a).createBindGroup(c(b));return g(d)});b.wbg.__wbg_createPipelineLayout_2648fbc756354294=((a,b)=>{const d=c(a).createPipelineLayout(c(b));return g(d)});b.wbg.__wbg_createRenderPipeline_513576fa326b8ccf=((a,b)=>{const d=c(a).createRenderPipeline(c(b));return g(d)});b.wbg.__wbg_createComputePipeline_957ea1dbcd97e6de=((a,b)=>{const d=c(a).createComputePipeline(c(b));return g(d)});b.wbg.__wbg_createBuffer_90ac080c7cc1375d=((a,b)=>{const d=c(a).createBuffer(c(b));return g(d)});b.wbg.__wbg_createTexture_4297303d703376ef=((a,b)=>{const d=c(a).createTexture(c(b));return g(d)});b.wbg.__wbg_createSampler_e56450d56435986f=((a,b)=>{const d=c(a).createSampler(c(b));return g(d)});b.wbg.__wbg_createQuerySet_c6b5390470139efb=((a,b)=>{const d=c(a).createQuerySet(c(b));return g(d)});b.wbg.__wbg_createCommandEncoder_9ee63be2a93c77dd=((a,b)=>{const d=c(a).createCommandEncoder(c(b));return g(d)});b.wbg.__wbg_createRenderBundleEncoder_bbce060a45e55caf=((a,b)=>{const d=c(a).createRenderBundleEncoder(c(b));return g(d)});b.wbg.__wbg_destroy_6e1daab7792230a0=(a=>{c(a).destroy()});b.wbg.__wbg_setonuncapturederror_0901d4d8bff41810=((a,b)=>{c(a).onuncapturederror=c(b)});b.wbg.__wbg_pushErrorScope_d39727ef0414ac9f=((a,b)=>{c(a).pushErrorScope(f(b))});b.wbg.__wbg_popErrorScope_1d998d85c7b134be=(a=>{const b=c(a).popErrorScope();return g(b)});b.wbg.__wbg_mapAsync_7d9fc5c22fb1f55e=((a,b,d,e)=>{const f=c(a).mapAsync(b>>>T,d,e);return g(f)});b.wbg.__wbg_unmap_abe29e47be94736f=(a=>{c(a).unmap()});b.wbg.__wbg_createView_8463cbef5f0c4d5c=((a,b)=>{const d=c(a).createView(c(b));return g(d)});b.wbg.__wbg_destroy_b8ea7d8b8cee78c4=(a=>{c(a).destroy()});b.wbg.__wbg_destroy_7fe69567d342b339=(a=>{c(a).destroy()});b.wbg.__wbg_getBindGroupLayout_255eaa69c120a995=((a,b)=>{const d=c(a).getBindGroupLayout(b>>>T);return g(d)});b.wbg.__wbg_getBindGroupLayout_d573a4d2adfb5ae8=((a,b)=>{const d=c(a).getBindGroupLayout(b>>>T);return g(d)});b.wbg.__wbg_copyBufferToBuffer_0a44e23b31a7ca5a=((a,b,d,e,f,g)=>{c(a).copyBufferToBuffer(c(b),d,c(e),f,g)});b.wbg.__wbg_copyBufferToTexture_de6f3cd9ac87a870=((a,b,d,e)=>{c(a).copyBufferToTexture(c(b),c(d),c(e))});b.wbg.__wbg_copyTextureToBuffer_7ab49ff0dd12cd22=((a,b,d,e)=>{c(a).copyTextureToBuffer(c(b),c(d),c(e))});b.wbg.__wbg_copyTextureToTexture_45800f5fb0aaaf6c=((a,b,d,e)=>{c(a).copyTextureToTexture(c(b),c(d),c(e))});b.wbg.__wbg_beginComputePass_99e2aa27fb960fa5=((a,b)=>{const d=c(a).beginComputePass(c(b));return g(d)});b.wbg.__wbg_end_a895c7d0f47bb8e0=(a=>{c(a).end()});b.wbg.__wbg_beginRenderPass_b4c178a1fd787b5c=((a,b)=>{const d=c(a).beginRenderPass(c(b));return g(d)});b.wbg.__wbg_end_0fafe47bdc78c53d=(a=>{c(a).end()});b.wbg.__wbg_label_4956528ad99b1650=((b,d)=>{const e=c(d).label;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ P]=g;r()[b/$+ T]=f});b.wbg.__wbg_finish_cbd8e5d52fe81fd6=((a,b)=>{const d=c(a).finish(c(b));return g(d)});b.wbg.__wbg_finish_3cd844105a9de3e9=(a=>{const b=c(a).finish();return g(b)});b.wbg.__wbg_clearBuffer_50e1d3d029849fdb=((a,b,d)=>{c(a).clearBuffer(c(b),d)});b.wbg.__wbg_clearBuffer_157bab025583c473=((a,b,d,e)=>{c(a).clearBuffer(c(b),d,e)});b.wbg.__wbg_writeTimestamp_70875f22e698e86b=((a,b,d)=>{c(a).writeTimestamp(c(b),d>>>T)});b.wbg.__wbg_resolveQuerySet_8f696a33e8da099f=((a,b,d,e,f,g)=>{c(a).resolveQuerySet(c(b),d>>>T,e>>>T,c(f),g>>>T)});b.wbg.__wbg_finish_806df42c71c712c3=(a=>{const b=c(a).finish();return g(b)});b.wbg.__wbg_finish_55ef253db8a2e02a=((a,b)=>{const d=c(a).finish(c(b));return g(d)});b.wbg.__wbg_writeBuffer_b225dafa1a52c298=((a,b,d,e,f,g)=>{c(a).writeBuffer(c(b),d,c(e),f,g)});b.wbg.__wbg_usage_2e5ff7c87b5e9737=(a=>{const b=c(a).usage;return b});b.wbg.__wbg_size_7838da1244dcc49f=(a=>{const b=c(a).size;return b});b.wbg.__wbg_writeTexture_05b125d21ce9740e=((a,b,d,e,f)=>{c(a).writeTexture(c(b),c(d),c(e),c(f))});b.wbg.__wbg_copyExternalImageToTexture_5389ee5babf9d86f=((a,b,d,e)=>{c(a).copyExternalImageToTexture(c(b),c(d),c(e))});b.wbg.__wbg_setPipeline_9730cb37968bb3d1=((a,b)=>{c(a).setPipeline(c(b))});b.wbg.__wbg_setBindGroup_c11c5cfe30b7ec4a=((a,b,d)=>{c(a).setBindGroup(b>>>T,c(d))});b.wbg.__wbg_setBindGroup_0184ac17323d75b2=((a,b,d,e,f,g,h)=>{c(a).setBindGroup(b>>>T,c(d),E(e,f),g,h>>>T)});b.wbg.__wbg_dispatchWorkgroups_2190ad793cd27850=((a,b,d,e)=>{c(a).dispatchWorkgroups(b>>>T,d>>>T,e>>>T)});b.wbg.__wbg_dispatchWorkgroupsIndirect_cfc6272439398a21=((a,b,d)=>{c(a).dispatchWorkgroupsIndirect(c(b),d)});b.wbg.__wbg_setPipeline_b1e4ff4a2d89b8aa=((a,b)=>{c(a).setPipeline(c(b))});b.wbg.__wbg_setBindGroup_2054136f79b0fed9=((a,b,d)=>{c(a).setBindGroup(b>>>T,c(d))});b.wbg.__wbg_setBindGroup_7908d39626c7bcc5=((a,b,d,e,f,g,h)=>{c(a).setBindGroup(b>>>T,c(d),E(e,f),g,h>>>T)});b.wbg.__wbg_setIndexBuffer_4deca629ec05a510=((a,b,d,e)=>{c(a).setIndexBuffer(c(b),f(d),e)});b.wbg.__wbg_setIndexBuffer_ea5677e397c8df89=((a,b,d,e,g)=>{c(a).setIndexBuffer(c(b),f(d),e,g)});b.wbg.__wbg_setVertexBuffer_4c924a9cc335e437=((a,b,d,e)=>{c(a).setVertexBuffer(b>>>T,c(d),e)});b.wbg.__wbg_setVertexBuffer_0aca41ad007e04fc=((a,b,d,e,f)=>{c(a).setVertexBuffer(b>>>T,c(d),e,f)});b.wbg.__wbg_draw_2ea14b17b7ad7b86=((a,b,d,e,f)=>{c(a).draw(b>>>T,d>>>T,e>>>T,f>>>T)});b.wbg.__wbg_drawIndexed_81f7662bc9f8bda1=((a,b,d,e,f,g)=>{c(a).drawIndexed(b>>>T,d>>>T,e>>>T,f,g>>>T)});b.wbg.__wbg_drawIndirect_3de3a4df802f8f74=((a,b,d)=>{c(a).drawIndirect(c(b),d)});b.wbg.__wbg_drawIndexedIndirect_74e31bc5d14e7aab=((a,b,d)=>{c(a).drawIndexedIndirect(c(b),d)});b.wbg.__wbg_setPipeline_d3556629635bf281=((a,b)=>{c(a).setPipeline(c(b))});b.wbg.__wbg_setBindGroup_4147d4ebb7213bb3=((a,b,d)=>{c(a).setBindGroup(b>>>T,c(d))});b.wbg.__wbg_setBindGroup_96a4847ff3077350=((a,b,d,e,f,g,h)=>{c(a).setBindGroup(b>>>T,c(d),E(e,f),g,h>>>T)});b.wbg.__wbg_setIndexBuffer_1860608e395ec140=((a,b,d,e)=>{c(a).setIndexBuffer(c(b),f(d),e)});b.wbg.__wbg_setIndexBuffer_83f311a5a378a545=((a,b,d,e,g)=>{c(a).setIndexBuffer(c(b),f(d),e,g)});b.wbg.__wbg_setVertexBuffer_d439a224a2369412=((a,b,d,e)=>{c(a).setVertexBuffer(b>>>T,c(d),e)});b.wbg.__wbg_setVertexBuffer_0dca9fc7421bd152=((a,b,d,e,f)=>{c(a).setVertexBuffer(b>>>T,c(d),e,f)});b.wbg.__wbg_draw_7266fe228aea02a8=((a,b,d,e,f)=>{c(a).draw(b>>>T,d>>>T,e>>>T,f>>>T)});b.wbg.__wbg_drawIndexed_23bcd62668716ed0=((a,b,d,e,f,g)=>{c(a).drawIndexed(b>>>T,d>>>T,e>>>T,f,g>>>T)});b.wbg.__wbg_drawIndirect_1a15176b1b8537ff=((a,b,d)=>{c(a).drawIndirect(c(b),d)});b.wbg.__wbg_drawIndexedIndirect_6f3721f18ad10b1e=((a,b,d)=>{c(a).drawIndexedIndirect(c(b),d)});b.wbg.__wbg_setBlendConstant_a946e294911337e9=((a,b)=>{c(a).setBlendConstant(c(b))});b.wbg.__wbg_setScissorRect_cd8f44130fd71416=((a,b,d,e,f)=>{c(a).setScissorRect(b>>>T,d>>>T,e>>>T,f>>>T)});b.wbg.__wbg_setViewport_66dfe2ad99a0ccd6=((a,b,d,e,f,g,h)=>{c(a).setViewport(b,d,e,f,g,h)});b.wbg.__wbg_setStencilReference_08db4d5601a3f285=((a,b)=>{c(a).setStencilReference(b>>>T)});b.wbg.__wbg_executeBundles_4bcd6c8ecfaedf51=((a,b)=>{c(a).executeBundles(c(b))});b.wbg.__wbg_submit_c512d9a4b5ff838d=((a,b)=>{c(a).submit(c(b))});b.wbg.__wbg_queueMicrotask_481971b0d87f3dd4=(a=>{queueMicrotask(c(a))});b.wbg.__wbg_queueMicrotask_3cbae2ec6b6cd3d6=(a=>{const b=c(a).queueMicrotask;return g(b)});b.wbg.__wbindgen_is_function=(a=>{const b=typeof c(a)===V;return b});b.wbg.__wbg_instanceof_Window_f401953a2cf86220=(a=>{let b;try{b=c(a) instanceof Window}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_document_5100775d18896c16=(a=>{const b=c(a).document;return p(b)?T:g(b)});b.wbg.__wbg_location_2951b5ee34f19221=(a=>{const b=c(a).location;return g(b)});b.wbg.__wbg_navigator_6c8fa55c5cc8796e=(a=>{const b=c(a).navigator;return g(b)});b.wbg.__wbg_innerHeight_c1ef73925c3d3e9c=function(){return F((a=>{const b=c(a).innerHeight;return g(b)}),arguments)};b.wbg.__wbg_devicePixelRatio_efc553b59506f64c=(a=>{const b=c(a).devicePixelRatio;return b});b.wbg.__wbg_localStorage_e381d34d0c40c761=function(){return F((a=>{const b=c(a).localStorage;return p(b)?T:g(b)}),arguments)};b.wbg.__wbg_performance_3298a9628a5c8aa4=(a=>{const b=c(a).performance;return p(b)?T:g(b)});b.wbg.__wbg_isSecureContext_3dd59a5324a1c6d5=(a=>{const b=c(a).isSecureContext;return b});b.wbg.__wbg_matchMedia_66bb21e3ef19270c=function(){return F(((a,b,d)=>{const e=c(a).matchMedia(k(b,d));return p(e)?T:g(e)}),arguments)};b.wbg.__wbg_open_cc82b8aaf0c296c1=function(){return F(((a,b,d,e,f)=>{const h=c(a).open(k(b,d),k(e,f));return p(h)?T:g(h)}),arguments)};b.wbg.__wbg_cancelAnimationFrame_111532f326e480af=function(){return F(((a,b)=>{c(a).cancelAnimationFrame(b)}),arguments)};b.wbg.__wbg_requestAnimationFrame_549258cfa66011f0=function(){return F(((a,b)=>{const d=c(a).requestAnimationFrame(c(b));return d}),arguments)};b.wbg.__wbg_clearInterval_4368213fd2b325b0=((a,b)=>{c(a).clearInterval(b)});b.wbg.__wbg_setTimeout_c172d5704ef82276=function(){return F(((a,b,d)=>{const e=c(a).setTimeout(c(b),d);return e}),arguments)};b.wbg.__wbg_body_edb1908d3ceff3a1=(a=>{const b=c(a).body;return p(b)?T:g(b)});b.wbg.__wbg_createElement_8bae7856a4bb7411=function(){return F(((a,b,d)=>{const e=c(a).createElement(k(b,d));return g(e)}),arguments)};b.wbg.__wbg_getElementById_c369ff43f0db99cf=((a,b,d)=>{const e=c(a).getElementById(k(b,d));return p(e)?T:g(e)});b.wbg.__wbg_querySelectorAll_4e0fcdb64cda2cd5=function(){return F(((a,b,d)=>{const e=c(a).querySelectorAll(k(b,d));return g(e)}),arguments)};b.wbg.__wbg_setid_37bacc3f09f555aa=((a,b,d)=>{c(a).id=k(b,d)});b.wbg.__wbg_scrollLeft_d34126a225a7a3dd=(a=>{const b=c(a).scrollLeft;return b});b.wbg.__wbg_clientWidth_7ea3915573b64350=(a=>{const b=c(a).clientWidth;return b});b.wbg.__wbg_clientHeight_d24efa25aa66e844=(a=>{const b=c(a).clientHeight;return b});b.wbg.__wbg_getBoundingClientRect_91e6d57c4e65f745=(a=>{const b=c(a).getBoundingClientRect();return g(b)});b.wbg.__wbg_scrollTop_323466d6f60b94d8=(a=>{const b=c(a).scrollTop;return b});b.wbg.__wbg_hidden_2da07df17092ca44=(a=>{const b=c(a).hidden;return b});b.wbg.__wbg_sethidden_1da7d3202cfe66f3=((a,b)=>{c(a).hidden=b!==T});b.wbg.__wbg_style_c3fc3dd146182a2d=(a=>{const b=c(a).style;return g(b)});b.wbg.__wbg_offsetTop_d164bbc281f71e80=(a=>{const b=c(a).offsetTop;return b});b.wbg.__wbg_offsetLeft_f8785f97dde57216=(a=>{const b=c(a).offsetLeft;return b});b.wbg.__wbg_offsetWidth_f7da5da36bd7ebc2=(a=>{const b=c(a).offsetWidth;return b});b.wbg.__wbg_blur_51f7b635f18a0eec=function(){return F((a=>{c(a).blur()}),arguments)};b.wbg.__wbg_focus_39d4b8ba8ff9df14=function(){return F((a=>{c(a).focus()}),arguments)};b.wbg.__wbg_navigator_56803b85352a0575=(a=>{const b=c(a).navigator;return g(b)});b.wbg.__wbg_instanceof_HtmlInputElement_307512fe1252c849=(a=>{let b;try{b=c(a) instanceof HTMLInputElement}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_setautofocus_5ef4f6fab60cacbf=((a,b)=>{c(a).autofocus=b!==T});b.wbg.__wbg_setsize_1e19966d9fce167e=((a,b)=>{c(a).size=b>>>T});b.wbg.__wbg_value_47fe6384562f52ab=((b,d)=>{const e=c(d).value;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ P]=g;r()[b/$+ T]=f});b.wbg.__wbg_setvalue_78cb4f1fef58ae98=((a,b,d)=>{c(a).value=k(b,d)});b.wbg.__wbg_writeText_4f1bf9bc5850bc26=((a,b,d)=>{const e=c(a).writeText(k(b,d));return g(e)});b.wbg.__wbg_matches_e14ed9ff8291cf24=(a=>{const b=c(a).matches;return b});b.wbg.__wbg_clientX_fef6bf7a6bcf41b8=(a=>{const b=c(a).clientX;return b});b.wbg.__wbg_clientY_df42f8fceab3cef2=(a=>{const b=c(a).clientY;return b});b.wbg.__wbg_ctrlKey_008695ce60a588f5=(a=>{const b=c(a).ctrlKey;return b});b.wbg.__wbg_shiftKey_1e76dbfcdd36a4b4=(a=>{const b=c(a).shiftKey;return b});b.wbg.__wbg_altKey_07da841b54bd3ed6=(a=>{const b=c(a).altKey;return b});b.wbg.__wbg_metaKey_86bfd3b0d3a8083f=(a=>{const b=c(a).metaKey;return b});b.wbg.__wbg_button_367cdc7303e3cf9b=(a=>{const b=c(a).button;return b});b.wbg.__wbg_now_4e659b3d15f470d9=(a=>{const b=c(a).now();return b});b.wbg.__wbg_data_1d8005e6d66d881b=((b,d)=>{const e=c(d).data;var f=p(e)?T:o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var g=l;r()[b/$+ P]=g;r()[b/$+ T]=f});b.wbg.__wbg_width_b455dec2a8f76e45=(a=>{const b=c(a).width;return b});b.wbg.__wbg_height_424ebb12c15f2691=(a=>{const b=c(a).height;return b});b.wbg.__wbg_preventDefault_b1a4aafc79409429=(a=>{c(a).preventDefault()});b.wbg.__wbg_stopPropagation_fa5b666049c9fd02=(a=>{c(a).stopPropagation()});b.wbg.__wbg_parentElement_347524db59fc2976=(a=>{const b=c(a).parentElement;return p(b)?T:g(b)});b.wbg.__wbg_appendChild_580ccb11a660db68=function(){return F(((a,b)=>{const d=c(a).appendChild(c(b));return g(d)}),arguments)};b.wbg.__wbg_get_8cd5eba00ab6304f=((a,b)=>{const d=c(a)[b>>>T];return p(d)?T:g(d)});b.wbg.__wbg_identifier_02d52b63cc6ddc4d=(a=>{const b=c(a).identifier;return b});b.wbg.__wbg_pageX_fa02f6046f16d09a=(a=>{const b=c(a).pageX;return b});b.wbg.__wbg_pageY_d1a7e78ba5b4cc5c=(a=>{const b=c(a).pageY;return b});b.wbg.__wbg_force_139077aa422a42a5=(a=>{const b=c(a).force;return b});b.wbg.__wbg_addEventListener_53b787075bd5e003=function(){return F(((a,b,d,e)=>{c(a).addEventListener(k(b,d),c(e))}),arguments)};b.wbg.__wbg_removeEventListener_92cb9b3943463338=function(){return F(((a,b,d,e)=>{c(a).removeEventListener(k(b,d),c(e))}),arguments)};b.wbg.__wbg_href_706b235ecfe6848c=function(){return F(((b,d)=>{const e=c(d).href;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ P]=g;r()[b/$+ T]=f}),arguments)};b.wbg.__wbg_origin_ee93e29ace71f568=function(){return F(((b,d)=>{const e=c(d).origin;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ P]=g;r()[b/$+ T]=f}),arguments)};b.wbg.__wbg_protocol_b7292c581cfe1e5c=function(){return F(((b,d)=>{const e=c(d).protocol;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ P]=g;r()[b/$+ T]=f}),arguments)};b.wbg.__wbg_host_8f1b8ead257c8135=function(){return F(((b,d)=>{const e=c(d).host;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ P]=g;r()[b/$+ T]=f}),arguments)};b.wbg.__wbg_hostname_3d9f22c60dc5bec6=function(){return F(((b,d)=>{const e=c(d).hostname;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ P]=g;r()[b/$+ T]=f}),arguments)};b.wbg.__wbg_port_b8d9a9c4e2b26efa=function(){return F(((b,d)=>{const e=c(d).port;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ P]=g;r()[b/$+ T]=f}),arguments)};b.wbg.__wbg_search_489f12953342ec1f=function(){return F(((b,d)=>{const e=c(d).search;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ P]=g;r()[b/$+ T]=f}),arguments)};b.wbg.__wbg_hash_553098e838e06c1d=function(){return F(((b,d)=>{const e=c(d).hash;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ P]=g;r()[b/$+ T]=f}),arguments)};b.wbg.__wbg_matches_dd4fdea75008ad05=(a=>{const b=c(a).matches;return b});b.wbg.__wbg_getItem_164e8e5265095b87=function(){return F(((b,d,e,f)=>{const g=c(d).getItem(k(e,f));var h=p(g)?T:o(g,a.__wbindgen_malloc,a.__wbindgen_realloc);var i=l;r()[b/$+ P]=i;r()[b/$+ T]=h}),arguments)};b.wbg.__wbg_setItem_ba2bb41d73dac079=function(){return F(((a,b,d,e,f)=>{c(a).setItem(k(b,d),k(e,f))}),arguments)};b.wbg.__wbg_items_5070ce38a6d53ed2=(a=>{const b=c(a).items;return g(b)});b.wbg.__wbg_files_a2848a7a7424820f=(a=>{const b=c(a).files;return p(b)?T:g(b)});b.wbg.__wbg_getData_35c5974f5cd7e02c=function(){return F(((b,d,e,f)=>{const g=c(d).getData(k(e,f));const h=o(g,a.__wbindgen_malloc,a.__wbindgen_realloc);const i=l;r()[b/$+ P]=i;r()[b/$+ T]=h}),arguments)};b.wbg.__wbg_keyCode_2af7775f99bf8e33=(a=>{const b=c(a).keyCode;return b});b.wbg.__wbg_altKey_2e6c34c37088d8b1=(a=>{const b=c(a).altKey;return b});b.wbg.__wbg_ctrlKey_bb5b6fef87339703=(a=>{const b=c(a).ctrlKey;return b});b.wbg.__wbg_shiftKey_5911baf439ab232b=(a=>{const b=c(a).shiftKey;return b});b.wbg.__wbg_metaKey_6bf4ae4e83a11278=(a=>{const b=c(a).metaKey;return b});b.wbg.__wbg_isComposing_a0b97b7ba6491ed6=(a=>{const b=c(a).isComposing;return b});b.wbg.__wbg_key_dccf9e8aa1315a8e=((b,d)=>{const e=c(d).key;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ P]=g;r()[b/$+ T]=f});b.wbg.__wbg_touches_c0f077e3c2429577=(a=>{const b=c(a).touches;return g(b)});b.wbg.__wbg_changedTouches_d044c818dbcb83b1=(a=>{const b=c(a).changedTouches;return g(b)});b.wbg.__wbg_length_679e0f1f9f0744bd=(a=>{const b=c(a).length;return b});b.wbg.__wbg_item_2b1028b3d39463e9=((a,b)=>{const d=c(a).item(b>>>T);return p(d)?T:g(d)});b.wbg.__wbg_get_cbca0027ab731230=((a,b)=>{const d=c(a)[b>>>T];return p(d)?T:g(d)});b.wbg.__wbg_top_c4e2234a035a3d25=(a=>{const b=c(a).top;return b});b.wbg.__wbg_left_fe0a839abdd508f4=(a=>{const b=c(a).left;return b});b.wbg.__wbg_setwidth_83d936c4b04dcbec=((a,b)=>{c(a).width=b>>>T});b.wbg.__wbg_setheight_6025ba0d58e6cc8c=((a,b)=>{c(a).height=b>>>T});b.wbg.__wbg_getContext_c102f659d540d068=function(){return F(((a,b,d)=>{const e=c(a).getContext(k(b,d));return p(e)?T:g(e)}),arguments)};b.wbg.__wbg_size_9c7e57fbd4f0f4b5=(a=>{const b=c(a).size;return b});b.wbg.__wbg_type_020d4abf13839639=((b,d)=>{const e=c(d).type;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ P]=g;r()[b/$+ T]=f});b.wbg.__wbg_arrayBuffer_307ddd1bd1d04e23=(a=>{const b=c(a).arrayBuffer();return g(b)});b.wbg.__wbg_name_f35eb93a73d94973=((b,d)=>{const e=c(d).name;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ P]=g;r()[b/$+ T]=f});b.wbg.__wbg_lastModified_e774a1d2d0384c3b=(a=>{const b=c(a).lastModified;return b});b.wbg.__wbg_instanceof_HtmlCanvasElement_46bdbf323b0b18d1=(a=>{let b;try{b=c(a) instanceof HTMLCanvasElement}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_width_aee8b8809b033b05=(a=>{const b=c(a).width;return b});b.wbg.__wbg_setwidth_080107476e633963=((a,b)=>{c(a).width=b>>>T});b.wbg.__wbg_height_80053d3c71b338e0=(a=>{const b=c(a).height;return b});b.wbg.__wbg_setheight_dc240617639f1f51=((a,b)=>{c(a).height=b>>>T});b.wbg.__wbg_getContext_df50fa48a8876636=function(){return F(((a,b,d)=>{const e=c(a).getContext(k(b,d));return p(e)?T:g(e)}),arguments)};b.wbg.__wbg_deltaX_206576827ededbe5=(a=>{const b=c(a).deltaX;return b});b.wbg.__wbg_deltaY_032e327e216f2b2b=(a=>{const b=c(a).deltaY;return b});b.wbg.__wbg_deltaMode_294b2eaf54047265=(a=>{const b=c(a).deltaMode;return b});b.wbg.__wbg_setProperty_ea7d15a2b591aa97=function(){return F(((a,b,d,e,f)=>{c(a).setProperty(k(b,d),k(e,f))}),arguments)};b.wbg.__wbg_type_e55aae30eb601b13=((b,d)=>{const e=c(d).type;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ P]=g;r()[b/$+ T]=f});b.wbg.__wbg_length_4db38705d5c8ba2f=(a=>{const b=c(a).length;return b});b.wbg.__wbg_get_58f6d5f6aee3f846=((a,b)=>{const d=c(a)[b>>>T];return p(d)?T:g(d)});b.wbg.__wbg_clipboardData_0427b2003659865a=(a=>{const b=c(a).clipboardData;return p(b)?T:g(b)});b.wbg.__wbg_length_a23c520109d9ba0a=(a=>{const b=c(a).length;return b});b.wbg.__wbg_get_0fa6ec8bd6a5c256=((a,b)=>{const d=c(a)[b>>>T];return p(d)?T:g(d)});b.wbg.__wbg_clipboard_45ef2514e9ece120=(a=>{const b=c(a).clipboard;return p(b)?T:g(b)});b.wbg.__wbg_userAgent_e94c7cbcdac01fea=function(){return F(((b,d)=>{const e=c(d).userAgent;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ P]=g;r()[b/$+ T]=f}),arguments)};b.wbg.__wbg_dataTransfer_cef7816623bd8478=(a=>{const b=c(a).dataTransfer;return p(b)?T:g(b)});b.wbg.__wbg_crypto_58f13aa23ffcb166=(a=>{const b=c(a).crypto;return g(b)});b.wbg.__wbg_process_5b786e71d465a513=(a=>{const b=c(a).process;return g(b)});b.wbg.__wbg_versions_c2ab80650590b6a2=(a=>{const b=c(a).versions;return g(b)});b.wbg.__wbg_node_523d7bd03ef69fba=(a=>{const b=c(a).node;return g(b)});b.wbg.__wbindgen_is_string=(a=>{const b=typeof c(a)===X;return b});b.wbg.__wbg_msCrypto_abcb1295e768d1f2=(a=>{const b=c(a).msCrypto;return g(b)});b.wbg.__wbg_require_2784e593a4674877=function(){return F((()=>{const a=module.require;return g(a)}),arguments)};b.wbg.__wbg_randomFillSync_a0d98aa11c81fe89=function(){return F(((a,b)=>{c(a).randomFillSync(f(b))}),arguments)};b.wbg.__wbg_getRandomValues_504510b5564925af=function(){return F(((a,b)=>{c(a).getRandomValues(c(b))}),arguments)};b.wbg.__wbg_new_16b304a2cfa7ff4a=(()=>{const a=new M();return g(a)});b.wbg.__wbg_newnoargs_e258087cd0daa0ea=((a,b)=>{const c=new Function(k(a,b));return g(c)});b.wbg.__wbg_call_27c0f87801dedf93=function(){return F(((a,b)=>{const d=c(a).call(c(b));return g(d)}),arguments)};b.wbg.__wbg_new_72fb9a18b5ae2624=(()=>{const a=new a0();return g(a)});b.wbg.__wbg_self_ce0dbfc45cf2f5be=function(){return F((()=>{const a=self.self;return g(a)}),arguments)};b.wbg.__wbg_window_c6fb939a7f436783=function(){return F((()=>{const a=window.window;return g(a)}),arguments)};b.wbg.__wbg_globalThis_d1e6af4856ba331b=function(){return F((()=>{const a=globalThis.globalThis;return g(a)}),arguments)};b.wbg.__wbg_global_207b558942527489=function(){return F((()=>{const a=global.global;return g(a)}),arguments)};b.wbg.__wbg_push_a5b05aedc7234f9f=((a,b)=>{const d=c(a).push(c(b));return d});b.wbg.__wbg_call_b3ca7c6051f9bec1=function(){return F(((a,b,d)=>{const e=c(a).call(c(b),c(d));return g(e)}),arguments)};b.wbg.__wbg_now_3014639a94423537=(()=>{const a=Date.now();return a});b.wbg.__wbg_instanceof_Object_71ca3c0a59266746=(a=>{let b;try{b=c(a) instanceof a0}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_valueOf_a0b7c836f68a054b=(a=>{const b=c(a).valueOf();return g(b)});b.wbg.__wbg_resolve_b0083a7967828ec8=(a=>{const b=Promise.resolve(c(a));return g(b)});b.wbg.__wbg_then_0c86a60e8fcfe9f6=((a,b)=>{const d=c(a).then(c(b));return g(d)});b.wbg.__wbg_then_a73caa9a87991566=((a,b,d)=>{const e=c(a).then(c(b),c(d));return g(e)});b.wbg.__wbg_buffer_12d079cc21e14bdb=(a=>{const b=c(a).buffer;return g(b)});b.wbg.__wbg_newwithbyteoffsetandlength_aa4a17c33a06e5cb=((a,b,d)=>{const e=new U(c(a),b>>>T,d>>>T);return g(e)});b.wbg.__wbg_new_63b92bc8671ed464=(a=>{const b=new U(c(a));return g(b)});b.wbg.__wbg_set_a47bac70306a19a7=((a,b,d)=>{c(a).set(c(b),d>>>T)});b.wbg.__wbg_length_c20a40f15020d68a=(a=>{const b=c(a).length;return b});b.wbg.__wbg_newwithlength_e9b4878cebadb3d3=(a=>{const b=new U(a>>>T);return g(b)});b.wbg.__wbg_buffer_dd7f74bc60f1faab=(a=>{const b=c(a).buffer;return g(b)});b.wbg.__wbg_subarray_a1f73cd4b5b42fe1=((a,b,d)=>{const e=c(a).subarray(b>>>T,d>>>T);return g(e)});b.wbg.__wbg_set_1f9b04f170055d33=function(){return F(((a,b,d)=>{const e=Reflect.set(c(a),c(b),c(d));return e}),arguments)};b.wbg.__wbindgen_debug_string=((b,d)=>{const e=u(c(d));const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ P]=g;r()[b/$+ T]=f});b.wbg.__wbindgen_throw=((a,b)=>{throw new S(k(a,b))});b.wbg.__wbindgen_memory=(()=>{const b=a.memory;return g(b)});b.wbg.__wbindgen_closure_wrapper8966=((a,b,c)=>{const d=w(a,b,a1,x);return g(d)});b.wbg.__wbindgen_closure_wrapper8968=((a,b,c)=>{const d=w(a,b,a1,y);return g(d)});b.wbg.__wbindgen_closure_wrapper8970=((a,b,c)=>{const d=w(a,b,a1,z);return g(d)});b.wbg.__wbindgen_closure_wrapper9767=((a,b,c)=>{const d=w(a,b,a2,A);return g(d)});b.wbg.__wbindgen_closure_wrapper9769=((a,b,c)=>{const d=w(a,b,a2,A);return g(d)});b.wbg.__wbindgen_closure_wrapper9868=((a,b,c)=>{const d=w(a,b,7716,B);return g(d)});return b});var B=((b,c,d)=>{a._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h137a324fc91c6fac(b,c,g(d))});var o=((a,b,c)=>{if(c===N){const c=m.encode(a);const d=b(c.length,P)>>>T;j().subarray(d,d+ c.length).set(c);l=c.length;return d};let d=a.length;let e=b(d,P)>>>T;const f=j();let g=T;for(;g<d;g++){const b=a.charCodeAt(g);if(b>127)break;f[e+ g]=b};if(g!==d){if(g!==T){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,P)>>>T;const b=j().subarray(e+ g,e+ d);const f=n(a,b);g+=f.written;e=c(e,d,g,P)>>>T};l=g;return e});var D=(()=>{if(C===O||C.byteLength===T){C=new Uint32Array(a.memory.buffer)};return C});var G=(async(a,b)=>{if(typeof Response===V&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===V){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var f=(a=>{const b=c(a);e(a);return b});var r=(()=>{if(q===O||q.byteLength===T){q=new Int32Array(a.memory.buffer)};return q});var y=((b,c)=>{try{const g=a.__wbindgen_add_to_stack_pointer(-_);a.wasm_bindgen__convert__closures__invoke0_mut__h21268eb6efc4c217(g,b,c);var d=r()[g/$+ T];var e=r()[g/$+ P];if(e){throw f(d)}}finally{a.__wbindgen_add_to_stack_pointer(_)}});var k=((a,b)=>{a=a>>>T;return h.decode(j().subarray(a,a+ b))});var I=((a,b)=>{});var J=((b,c)=>{a=b.exports;L.__wbindgen_wasm_module=c;s=O;q=O;C=O;i=O;a.__wbindgen_start();return a});let a;const b=new M(128).fill(N);b.push(N,O,!0,!1);let d=b.length;const h=typeof TextDecoder!==Q?new TextDecoder(R,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw S(`TextDecoder not available`)}};if(typeof TextDecoder!==Q){h.decode()};let i=O;let l=T;const m=typeof TextEncoder!==Q?new TextEncoder(R):{encode:()=>{throw S(`TextEncoder not available`)}};const n=typeof m.encodeInto===V?((a,b)=>m.encodeInto(a,b)):((a,b)=>{const c=m.encode(a);b.set(c);return {read:a.length,written:c.length}});let q=O;let s=O;const v=typeof Z===Q?{register:()=>{},unregister:()=>{}}:new Z(b=>{a.__wbindgen_export_2.get(b.dtor)(b.a,b.b)});let C=O;export default L;export{K as initSync}