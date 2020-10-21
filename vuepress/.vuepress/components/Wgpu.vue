<template>
    <div id="wgpu-display">
        <!-- WebGPU content should go here -->
    </div>
</template>
<script>
// import { demo } from "wgpu-multiplatform"
export default {
    mounted() {
        import("wgpu-multiplatform").then(module => {
            console.log(module);
            module.demo();
        })
    }
}
</script>