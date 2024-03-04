<template>
    <div v-if="game_type == 'Archipelago'" class="archipelago">
        <img src="https://archipelago.gg/static/static/branding/header-logo.svg" />
    </div>
    <div v-else-if="game_type == 'A Link to the Past Randomizer'" class="alttpr">
        <img src="https://alttpr.com/i/logo.png" />
    </div>
    <div v-else>
        {{ game_type }}
    </div>
</template>

<style lang="scss">
.archipelago {
    border-radius: 8px;
    box-sizing: border-box;
    display: grid;
    height: 64px;
    max-height: 64px;
    padding: 0 16px;
    place-content: center;

    /* Image source offsets. */
    img {
        height: 100%;
        position: relative;
        top: 4px;
        width: 100%;
    }
}

.alttpr {
    display: grid;
    box-sizing: border-box;
    height: 84px;
    max-height: 84px;
    padding: 0;
    place-content: center;
}
</style>

<script>

export default {
    props: ["game_type"],
}

</script>