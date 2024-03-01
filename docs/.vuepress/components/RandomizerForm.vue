<template>
  <div v-if="state == 'INIT'">
    <p>Select a file, fill out the options you want, and hit Generate!</p>
    <input @change="fileSelect($event)" type="file">
  </div>

  <div v-if="state == 'FILE_SELECTED'" class="randomize-option-form">
    <div class="randomizer-option-field" role="presentation">
      <label for="seed">Seed</label>
      <input id="seed" v-model="seed" />
    </div>

    <div class="randomizer-option-field" role="presentation">
      <label for="shadow-bees">Shadow Bees</label>
      <input id="shadow-bees" type="checkbox" v-model="shadowBees" />
    </div>

    <div class="randomizer-option-field" role="presentation">
      <label for="boss-shuffle">Boss Shuffle</label>
      <input id="boss-shuffle" type="checkbox" v-model="bossShuffle" />
    </div>

    <div class="randomizer-option-field" role="presentation">
      <label for="mushrrom-shuffle">Mushroom Shuffle</label>
      <input id="mushrrom-shuffle" type="checkbox" v-model="mushroomShuffle" />
    </div>

    <div class="randomizer-option-field" role="presentation">
      <label for="ow-balancing">Overworld Balancing</label>
      <select id="ow-balancing" v-model="owBalancing">
        <option v-for="(display, value) in balancingOptions" :key="value" :value="value">{{ display }}</option>
      </select>
    </div>

    <div class="randomizer-option-field" role="presentation">
      <label for="ow-balancing">Overworld Enemy Shuffle</label>
      <select id="ow-balancing" v-model="owEnemyShuffle">
        <option v-for="(display, value) in owEnemyShuffleOptions" :key="value" :value="value">{{ display }}</option>
      </select>
    </div>

    <div class="randomizer-option-field" role="presentation">
      <label for="uw-balancing">Underworld Balancing</label>
      <select id="uw-balancing" v-model="uwBalancing">
        <option v-for="(display, value) in balancingOptions" :key="value" :value="value">{{ display }}</option>
      </select>
    </div>

    <div class="randomizer-option-field" role="presentation">
      <label for="ow-balancing">Underworld Enemy Shuffle</label>
      <select id="ow-balancing" v-model="uwEnemyShuffle">
        <option v-for="(display, value) in uwEnemyShuffleOptions" :key="value" :value="value">{{ display }}</option>
      </select>
    </div>


    <div class="randomize-option-footer">
      <button @click="processZelda3()">Generate!</button>
    </div>
  </div>

  <div v-if="state == 'ERROR'">
    <pre><code>{{ errorMessage }}</code></pre>
    <button @click="processZelda3()">Retry</button>
  </div>

  <div v-if="state == 'DONE'">
    <p>Have a wonderful journey!</p>
    <button @click="initialize()">Restart</button>
  </div>
</template>

<style lang="scss">
.randomize-option-form {
  display: grid;
  grid-template-columns: 1fr;
  gap: 12px;
  margin: 24px;
  max-width: 480px;
}

.randomizer-option-field {
  display: grid;
  grid-template-columns: 1fr auto;
}

.randomize-option-footer {
  margin: 16px 0;
  text-align: end;
}
</style>

<script>
import * as pkg from '../pkg';

const INIT = 'INIT';
const FILE_SELECTED = 'FILE_SELECTED';
const PROCESSING = 'processing';
const DONE = 'DONE';
const ERROR = 'ERROR';

export default {
  data() {
    return {
      // State properties
      state: INIT,
      errorMessage: '',
      timeElapsed: undefined,
      // Options
      balancingOptions: {},
      owEnemyShuffleOptions: {},
      uwEnemyShuffleOptions: {},
      // Properties to send to randomizer.
      buffer: undefined,
      seed: '',
      bossShuffle: false,
      mushroomShuffle: false,
      owBalancing: 0,
      owEnemyShuffle: 0,
      shadowBees: false,
      uwBalancing: 0,
      uwEnemyShuffle: 0,
    };
  },
  created() {
    pkg.init();

    this.balancingOptions = pkg.get_balancing_options();
    this.owEnemyShuffleOptions = pkg.get_ow_enemy_shuffle_options();
    this.uwEnemyShuffleOptions = pkg.get_uw_enemy_shuffle_options();
    this.initialize();
  },
  methods: {
    async fileSelect(evt) {
      let file = evt.target.files[0];
      return readFileAsBytes(file).then(bytes => {
        this.buffer = bytes;
        this.state = FILE_SELECTED;
      })
    },
    initialize() {
      this.state = INIT;
      this.errorMessage = "";
      this.timeElapsed = undefined;
      this.buffer = undefined;
      this.seed = generateUID();
      this.bossShuffle = false;
      this.mushroomShuffle = false;
      this.owBalancing = 0;
      this.owEnemyShuffle = 0;
      this.shadowBees = false;
      this.uwBalancing = 0;
      this.uwEnemyShuffle = 0;
    },
    processZelda3() {
      try {
        this.state = PROCESSING;
        this.errorMessage = '';

        const start = performance.now();
        const output = pkg.process_zelda3(
          this.seed,
          this.bossShuffle,
          this.mushroomShuffle,
          Number(this.owBalancing),
          Number(this.owEnemyShuffle),
          this.shadowBees,
          Number(this.uwBalancing),
          Number(this.uwEnemyShuffle),
          this.buffer,
        );

        this.timeElapsed = performance.now() - start;
        this.state = DONE;

        downloadByteArrayAsFile(output, 'download.sfc');
      } catch (e) {
        this.state = ERROR;
        this.errorMessage = e.stack;
      }
    },
  },
};

function generateUID() {
  const timestamp = Date.now().toString(36);
  const randomString = Math.random().toString(36).substring(2, 7);
  return timestamp + randomString;
}

function readFileAsBytes(file) {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => {
      const arrayBuffer = reader.result;
      const byteArray = new Uint8Array(arrayBuffer);
      resolve(byteArray);
    };
    reader.onerror = reject;
    reader.readAsArrayBuffer(file);
  });
}

function downloadByteArrayAsFile(byteArray, fileName) {
  const blob = new Blob([byteArray]);
  const url = URL.createObjectURL(blob);
  const link = document.createElement('a');
  link.href = url;
  link.download = fileName;
  link.click();
  URL.revokeObjectURL(url);
  link.remove();
}
</script>