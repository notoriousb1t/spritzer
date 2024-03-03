<template>
  <div v-if="state == 'INIT'">
    <p>Select a file, fill out the options you want, and hit Generate!</p>
    <input @change="fileSelect($event)" type="file">
  </div>

  <div v-if="state == 'FILE_SELECTED'" class="randomize-option-form">

    <a-input v-model:value="seed" addonBefore="Seed" placeholder="Please enter a random string" />


    <div class="randomizer-option-field" role="presentation">
      <label>Overworld Balancing</label>
      <a-select ref="select" v-model:value="owBalancing" style="width: 120px">
        <a-select-option v-for="option in balancingOptions" :value="option">{{ option }}</a-select-option>
      </a-select>
    </div>

    <div class="randomizer-option-field" role="presentation">
      <label>Overworld Enemy Shuffle</label>
      <a-select ref="select" v-model:value="owEnemyShuffle" style="width: 120px">
        <a-select-option v-for="option in owEnemyShuffleOptions" :value="option">{{ option }}</a-select-option>
      </a-select>
    </div>

    <div class="randomizer-option-field" role="presentation">
      <label>Underworld Balancing</label>
      <a-select ref="select" v-model:value="uwBalancing" style="width: 120px">
        <a-select-option v-for="option in balancingOptions" :value="option">{{ option }}</a-select-option>
      </a-select>
    </div>

    <div class="randomizer-option-field" role="presentation">
      <label>Underworld Enemy Shuffle</label>
      <a-select ref="select" v-model:value="uwEnemyShuffle" style="width: 120px">
        <a-select-option v-for="option in owEnemyShuffleOptions" :value="option">{{ option }}</a-select-option>
      </a-select>
    </div>

    <div>
      <br/> <!-- Gasp! -->
      <a-checkbox v-model:checked="shadowBees">Shadow Bees</a-checkbox>
      <a-checkbox v-model:checked="bossShuffle">Boss Shuffle</a-checkbox>
      <a-checkbox v-model:checked="mushroomShuffle">Mushroom Shuffle</a-checkbox>
    </div>

    <div class="randomize-option-footer">
      <a-button type="primary" @click="processZelda3()">Generate</a-button>
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
  align-items: center;
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
      balancingOptions: [],
      owEnemyShuffleOptions: [],
      uwEnemyShuffleOptions: [],
      // Properties to send to randomizer.
      buffer: undefined,
      seed: '',
      bossShuffle: false,
      mushroomShuffle: false,
      owBalancing: "Vanilla",
      owEnemyShuffle: 0,
      shadowBees: false,
      uwBalancing: 0,
      uwEnemyShuffle: 0,
    };
  },
  created() {
    pkg.init();

    let options = pkg.get_options();
    console.log(options);
    this.balancingOptions = options.balancingOptions;
    this.owEnemyShuffleOptions = options.owEnemyShuffleOptions;
    this.uwEnemyShuffleOptions = options.uwEnemyShuffleOptions;
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
      this.owBalancing = "Vanilla";
      this.owEnemyShuffle = "Random";
      this.shadowBees = false;
      this.uwBalancing = "Vanilla";
      this.uwEnemyShuffle = "Random";
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
          this.owBalancing,
          this.owEnemyShuffle,
          this.shadowBees,
          this.uwBalancing,
          this.uwEnemyShuffle,
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