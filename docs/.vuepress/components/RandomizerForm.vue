<template>
  <a-config-provider :theme="{
    token: {
      colorPrimary: '#127900',
    },
  }">
  </a-config-provider>

  <div v-if="state == 'INIT'">
    <FileSelect @file-selected="fileSelected($event)" />
  </div>

  <div v-if="state == 'FILE_SELECTED'" class="randomize-option-form">
    <template v-if="supported">
      <h1>Options</h1>

      <div class="randomizer-option-field" role="presentation">
        <label>Item Randomizer</label>
        <GameTypeImage :game_type />
      </div>
      <div class="randomizer-option-field" role="presentation">
        <label>Seed</label>
        <a-input v-model:value="opts.seed" addonBefore="⋆" placeholder="Please enter a random string" />
      </div>

      <div class="randomizer-option-field" role="presentation">
        <label>Overworld Balancing</label>
        <a-select ref="select" v-model:value="opts.ow_balancing">
          <a-select-option v-for="option in opts.balancing_options" :value="option">{{ option }}</a-select-option>
        </a-select>
      </div>

      <div class="randomizer-option-field" role="presentation">
        <label>Overworld Enemy Shuffle</label>
        <a-select ref="select" v-model:value="opts.ow_enemy_shuffle">
          <a-select-option v-for="option in opts.ow_enemy_shuffle_options" :value="option">{{ option
            }}</a-select-option>
        </a-select>
      </div>

      <div class="randomizer-option-field" role="presentation">
        <label>Underworld Balancing</label>
        <a-select ref="select" v-model:value="opts.uw_balancing">
          <a-select-option v-for="option in opts.balancing_options" :value="option">{{ option }}</a-select-option>
        </a-select>
      </div>

      <div class="randomizer-option-field" role="presentation">
        <label>Underworld Enemy Shuffle</label>
        <a-select ref="select" v-model:value="opts.uw_enemy_shuffle">
          <a-select-option v-for="option in opts.ow_enemy_shuffle_options" :value="option">{{ option
            }}</a-select-option>
        </a-select>
      </div>

      <div class="randomizer-option-field" role="presentation">
        <label>Shadow Bees</label>
        <div>
          <a-switch v-model:checked="opts.shadow_bees" />
        </div>
      </div>

      <div class="randomizer-option-field" role="presentation">
        <label>Boss Shuffle</label>
        <div>
          <a-switch v-model:checked="opts.boss_shuffle" />
        </div>
      </div>

      <div class="randomizer-option-field" role="presentation">
        <label>Killable Thieves</label>
        <div>
          <a-switch v-model:checked="opts.killable_thieves" />
        </div>
      </div>

      <div class="randomizer-option-field" role="presentation">
        <label>Mushroom Shuffle</label>
        <div>
          <a-switch v-model:checked="opts.mushroom_shuffle" />
        </div>
      </div>

      <div class="randomizer-option-field" role="presentation">
        <label>Overworld Inverted</label>
        <div>
          <a-switch v-model:checked="opts.overworld_inverted" />
        </div>
      </div>

      <div class="randomizer-option-field" role="presentation">
        <label>Pot Shuffle</label>
        <div>
          <a-switch v-model:checked="opts.pot_shuffle" />
        </div>
      </div>

      <div class="randomize-option-footer">
        <a-button type="primary" shape="round" size="large" @click="processZelda3()">Roll Game</a-button>
      </div>
    </template>

    <template v-else>
      <div class="unsupported">
        <h1>Sorry :(</h1>
        <GameTypeImage :game_type />
        <p>
          <em>This game/combination is not supported, yet</em>
        </p>
        <p>Check <a href="/spritzer/guide.html#unsupported-games">Unsupported Games</a> in the guide to troubleshoot
          this. You may need to remove some options from your Item Randomizer</p>
      </div>
    </template>
  </div>

  <div v-if="state == 'ERROR'">
    <h1>I AM ERROR</h1>
    <pre class="language-bash"><code>{{ errorMessage }}</code></pre>
    <a-button type="primary" @click="processZelda3()">Retry</a-button>
  </div>

  <div v-if="state == 'DONE'">
    <Confirmation @file-download="download()" />
  </div>
</template>

<style lang="scss">
.randomize-option-form {
  display: grid;
  grid-template-columns: 1fr;
  gap: 12px;
  margin: 24px;
  max-width: 680px;
  place-content: center;
}

.randomizer-option-field {
  align-items: center;
  display: grid;
  gap: 16px;
  grid-template-columns: minmax(auto, 240px) 1fr;

  @media (max-width: 560px) {
    grid-template-columns: 100%;
  }
}

.randomize-option-footer {
  margin: 16px 0;
  text-align: end;
}

.unsupported {
  text-align: center;
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
      game_type: '',
      supported: false,
      opts: {},
      input_name: "",
      input_buffer: undefined,
      output_buffer: undefined,
    };
  },
  mounted() {
    // TODO: Run this in enhance.
    if (!window.__init_spritzer) {
      window.__init_spritzer = true;
      pkg.init();
    }
  },
  methods: {
    fileSelected({ bytes, name }) {
      const result = pkg.detect_options(bytes);
      this.input_name = name;
      this.opts = result.options;
      this.supported = result.supported;
      this.game_type = result.game_type;
      this.input_buffer = bytes;
      this.state = FILE_SELECTED;
    },
    processZelda3() {
      try {
        this.state = PROCESSING;
        this.errorMessage = '';

        this.output_buffer = pkg.process_zelda3(this.input_buffer, this.opts);
        this.state = DONE;
        downloadByteArrayAsFile(this.output_buffer, this.input_name);
      } catch (e) {
        this.state = ERROR;
        this.errorMessage = e.stack;
      }
    },
    download() {
      downloadByteArrayAsFile(this.output_buffer, this.input_name);
    },
  },
};


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