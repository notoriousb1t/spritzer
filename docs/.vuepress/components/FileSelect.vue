<template>
    <div role="presentation" class="el">
        <h1>Select the game file</h1>
        <a-upload-dragger name="file" :before-upload="fileSelected">
            <div class="uploader-hint">
                <p class="ant-upload-text">Click this or drag file (.sfc, .smc) to this area</p>
                <p class="ant-upload-hint">
                    This application runs entirely in your browser. No files are uploaded to any server
                </p>
            </div>
        </a-upload-dragger>
    </div>
</template>

<style lang="scss">
.el {
    text-align: center;
}

.uploader-hint {
    padding: 32px;
}
</style>

<script>
export default {
    data() {
        return {
            fileList: [],
        };
    },
    emits: ['file-selected'],
    methods: {
        async fileSelected(file) {
            let bytes = await readFileAsBytes(file);
            this.$emit('file-selected', { bytes, name: file.name || 'download.sfc' });
            return false;
        },
    },
};


function readFileAsBytes(file) {
    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = () => {
            const byteArray = new Uint8Array(reader.result);
            resolve(byteArray);
        };
        reader.onerror = reject;
        reader.readAsArrayBuffer(file);
    });
}
</script>
