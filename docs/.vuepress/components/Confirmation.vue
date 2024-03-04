<template>
    <h1>Have a wonderful journey!</h1>
    <p>
        The randomized game has started downloading.
    </p>
    <a-button type="primary" @click="$emit('file-download')">Download Again</a-button>
</template>

<script>
export default {
    emits: ['file-download'],
};
</script>
