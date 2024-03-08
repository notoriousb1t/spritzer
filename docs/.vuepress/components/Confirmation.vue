<template>
    <div role="presentation" class="el">
        <h1>Have a wonderful journey!</h1>
        <p>
            The file has been saved to your Downloads folder
        </p>
        <a-button type="primary" @click="$emit('file-download')">Save Another Copy</a-button>
    </div>
</template>

<script>
export default {
    emits: ['file-download'],
};
</script>

<style lang="scss">
.el {
    text-align: center;
}
</style>