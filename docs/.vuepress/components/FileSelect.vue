<template>
    <h1>Select the game file</h1>
    <p>
        NOTE: This application runs entirely in your browser. No files are uploaded to any server.
    </p>
    <a-upload :before-upload="fileSelected">
        <a-button type="primary" shape="round" size="large">
            <upload-outlined></upload-outlined>
            Locate Game File (.sfc, .smc)
        </a-button>
    </a-upload>
</template>

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
            this.$emit('file-selected', bytes);
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
