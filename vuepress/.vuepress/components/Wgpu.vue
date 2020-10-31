<template>
    <div id="wgpu-display">
        <WgpuError v-if="wgpuErrorOccurred"></WgpuError>
        <Spinner v-if="loading"></Spinner>
        <!-- WebGPU content should go here -->
    </div>
</template>
<script>
import Spinner from 'vue-simple-spinner/src/components/Spinner'
export default {
    components: {
        Spinner,
    },
    data() {
        return {
            wgpuErrorOccurred: false,
            loading: false,
        }
    },
    mounted() {
        this.loading = true;
        
        import("demo").then(module => {
            console.debug("Loaded module!");
            console.debug(module);
            this.loading = false;

            try {
                module.demo(512, 512, "wgpu-display");
                console.log("No errors reported");
                this.wgpuErrorOccurred = false;
            } catch {
                console.log("An error occurred")
                this.wgpuErrorOccurred = true;
            }
        });
    }
}
</script>