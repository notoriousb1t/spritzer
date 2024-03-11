"use strict";(self.webpackChunkweb=self.webpackChunkweb||[]).push([[94],{9550:(e,a,l)=>{l.r(a),l.d(a,{comp:()=>p,data:()=>c});var n=l(641);const r=(0,n.Fv)('<h1 id="guide" tabindex="-1"><a class="header-anchor" href="#guide"><span>Guide</span></a></h1><p>Spritzer adds dungeon and sprite randomization to item randomizer games generated by Archipelago, etc. This game mod preserves all item logic present in the game file, but will switch the hazards, enemies, dungeon tilesets, and sometimes the NPCs to make the game fresher for fans of Zelda 3.</p><h2 id="options" tabindex="-1"><a class="header-anchor" href="#options"><span>Options</span></a></h2><h3 id="overworld-balancing" tabindex="-1"><a class="header-anchor" href="#overworld-balancing"><span>Overworld Balancing</span></a></h3><p>Balancing controls how difficult sprites, hazards, and consumables are for the player in the overworld. This affects rerolling of enemies, consumables, hazards, and traps (aka Overlords)</p><ol><li><strong>Random</strong> - There is an equal chance of swaps within a category.</li><li><strong>Casual</strong> - The game is biased to make the game easier for the player.</li><li><strong>Balanced</strong> - This generally should feel similar to the original game.</li><li><strong>Hero</strong> - The game is biased to make the game more difficult for the player.</li></ol><h3 id="overworld-enemy-shuffle" tabindex="-1"><a class="header-anchor" href="#overworld-enemy-shuffle"><span>Overworld Enemy Shuffle</span></a></h3><p>Enemy Shuffle controls which enemies/etc. appear within the overworld</p><ol><li><strong>Vanilla</strong> - Changes to the enemies placed are avoided</li><li><strong>Simple</strong> - Enemies are shuffled, but each screen should contain the same enemies overall.</li><li><strong>Full</strong> - Enemies are rerolled. Enemies that appear together are chosen at random.</li><li><strong>Inverted</strong> - Dark and light world enemies may be swapped and rerolled. Kakariko NPCs are moved to the dark world except for Bottle Salesman, who is moved to Lumberjack&#39;s house to preserve item logic.</li><li><strong>Chaos</strong> - TBD - planned to mix light and dark world indiscriminately</li><li><strong>Insanity</strong> - TBD - planned to indescriminately place hazards and enemies from any location.</li></ol><h3 id="killable-thieves" tabindex="-1"><a class="header-anchor" href="#killable-thieves"><span>Killable Thieves</span></a></h3><p>Makes thieves killable (one or two hits)</p><h3 id="mushroom-shuffle" tabindex="-1"><a class="header-anchor" href="#mushroom-shuffle"><span>Mushroom Shuffle</span></a></h3><p>Shuffles the location of the overworld Mushroom location in the Lost Woods. The item here may appear at any of the locations of Fake Swords or the original location at random.</p><h3 id="boss-shuffle-experimental" tabindex="-1"><a class="header-anchor" href="#boss-shuffle-experimental"><span>Boss Shuffle (Experimental)</span></a></h3><p>Shuffles the bosses while preserving item logic. Currently the dungeons always change to match the boss.</p><h3 id="underworld-balancing" tabindex="-1"><a class="header-anchor" href="#underworld-balancing"><span>Underworld Balancing</span></a></h3><p>Balancing controls how difficult sprites, hazards, and consumables are for the player in the underworld. This affects rerolling of enemies, consumables, hazards, and traps (aka Overlords)</p><ol><li><strong>Random</strong> - There is an equal chance of swaps within a category.</li><li><strong>Casual</strong> - The game is biased to make the game easier for the player.</li><li><strong>Balanced</strong> - This generally should feel similar to the original game.</li><li><strong>Hero</strong> - The game is biased to make the game more difficult for the player.</li></ol><h3 id="underworld-enemy-shuffle" tabindex="-1"><a class="header-anchor" href="#underworld-enemy-shuffle"><span>Underworld Enemy Shuffle</span></a></h3><p>Enemy Shuffle controls which enemies/etc. appear within the underworld (dungeons + caves)</p><ol><li><strong>Vanilla</strong> - Changes to the enemies placed are avoided</li><li><strong>Simple</strong> - Enemies are shuffled, but each supertile should contain the same enemies overall.</li><li><strong>Full</strong> - Enemies are rerolled. Enemies that appear together are chosen at random.</li><li><strong>Chaos</strong> - TBD - planned to mix enemies between dungeons</li><li><strong>Insanity</strong> - TBD - planned to indescriminately place hazards and enemies from any location</li></ol><h3 id="seed" tabindex="-1"><a class="header-anchor" href="#seed"><span>Seed</span></a></h3><p>Controls randomization. Is intended to produce the same result given the same general options.</p><blockquote><p>Some special values are present for debugging purposes. For example, if the word &quot;moldorm4&quot; appears in the seed, all bosses will change to Moldorm and Moldorm will have four eyes. Debug strings are not guaranteed to produce a winnable game. For example, agahnim will replace all bosses with agahnim but they currently do not drop boss prizes.</p></blockquote><h3 id="shadow-bees" tabindex="-1"><a class="header-anchor" href="#shadow-bees"><span>Shadow Bees</span></a></h3><p>Changes bees to be more powerful and partially invisible.</p><h2 id="other-world-changes" tabindex="-1"><a class="header-anchor" href="#other-world-changes"><span>Other World Changes</span></a></h2><p>Some features are enabled as a result of other enabled features:</p><ol><li><strong>Dungeon Shuffle</strong> - By default, shuffling bosses also shuffles the appearance and characteristics of the dungeons. In some cases, like Tower of Hera, the boss room may change appearance. Please file an issue, if there is demand to turn this off.</li><li><strong>Visual Enemy Changes</strong> - Some enemies have changed color palettes to be more stable across different locations. For example, mini-moldorm and Green Eyegore are blue to improve user experience across environments. Also Moldorm has a random number of eyes.</li><li><strong>Overworld Simplication</strong> - Enemies/NPCs will remain the same before and after killing Agahnim unless the room has a specific item logic. i.e. Hyrule Castle, Lumberjack&#39;s house, etc.</li></ol><h2 id="patching-archipelago" tabindex="-1"><a class="header-anchor" href="#patching-archipelago"><span>Patching Archipelago</span></a></h2>',30),s={href:"http://https://archipelago.gg",target:"_blank",rel:"noopener noreferrer"},o=(0,n.Lk)("li",null,"Download the patch file and run the alttp file. Do not close Archipelago or connect via Lua.",-1),i=(0,n.Lk)("li",null,"Replace the .sfc generated by Archipelago",-1),t=(0,n.Lk)("li",null,"Connect via Lua as normal",-1),h=(0,n.Fv)('<blockquote><p>TIP: Archipelago may replace the .sfc each time it is launched, so it is a good idea to keep a backup of the generated file if you need to restart.</p></blockquote><h2 id="unsupported-games" tabindex="-1"><a class="header-anchor" href="#unsupported-games"><span>Unsupported Games</span></a></h2><h3 id="archipelago-enemizer" tabindex="-1"><a class="header-anchor" href="#archipelago-enemizer"><span>Archipelago + Enemizer</span></a></h3><p>To make the game compatible, remove these options from the YAML configuration file and regenerate in Archipelago:</p><ol><li>boss_shuffle</li><li>enemy_shuffle</li><li>killable_thieves</li><li>enemy_health</li><li>enemy_damage</li><li>pot_shuffle</li><li>bush_shuffle</li></ol><h3 id="the-alttp-randomizer" tabindex="-1"><a class="header-anchor" href="#the-alttp-randomizer"><span>The ALTTP Randomizer</span></a></h3><p>(Needs additional testing)</p><h3 id="vanilla-alttp-games" tabindex="-1"><a class="header-anchor" href="#vanilla-alttp-games"><span>Vanilla ALTTP Games</span></a></h3><p>(Under consideration)</p>',9),d={},p=(0,l(6262).A)(d,[["render",function(e,a){const l=(0,n.g2)("ExternalLinkIcon"),d=(0,n.g2)("RouteLink");return(0,n.uX)(),(0,n.CE)("div",null,[r,(0,n.Lk)("ol",null,[(0,n.Lk)("li",null,[(0,n.eW)("Generate a game on "),(0,n.Lk)("a",s,[(0,n.eW)("Archipelago"),(0,n.bF)(l)])]),o,(0,n.Lk)("li",null,[(0,n.eW)("Generate the file in "),(0,n.bF)(d,{to:"/randomize.html"},{default:(0,n.k6)((()=>[(0,n.eW)("Randomize")])),_:1})]),i,t]),h])}]]),c=JSON.parse('{"path":"/guide.html","title":"Guide","lang":"en-US","frontmatter":{"sidebar":"auto"},"headers":[{"level":2,"title":"Options","slug":"options","link":"#options","children":[{"level":3,"title":"Overworld Balancing","slug":"overworld-balancing","link":"#overworld-balancing","children":[]},{"level":3,"title":"Overworld Enemy Shuffle","slug":"overworld-enemy-shuffle","link":"#overworld-enemy-shuffle","children":[]},{"level":3,"title":"Killable Thieves","slug":"killable-thieves","link":"#killable-thieves","children":[]},{"level":3,"title":"Mushroom Shuffle","slug":"mushroom-shuffle","link":"#mushroom-shuffle","children":[]},{"level":3,"title":"Boss Shuffle (Experimental)","slug":"boss-shuffle-experimental","link":"#boss-shuffle-experimental","children":[]},{"level":3,"title":"Underworld Balancing","slug":"underworld-balancing","link":"#underworld-balancing","children":[]},{"level":3,"title":"Underworld Enemy Shuffle","slug":"underworld-enemy-shuffle","link":"#underworld-enemy-shuffle","children":[]},{"level":3,"title":"Seed","slug":"seed","link":"#seed","children":[]},{"level":3,"title":"Shadow Bees","slug":"shadow-bees","link":"#shadow-bees","children":[]}]},{"level":2,"title":"Other World Changes","slug":"other-world-changes","link":"#other-world-changes","children":[]},{"level":2,"title":"Patching Archipelago","slug":"patching-archipelago","link":"#patching-archipelago","children":[]},{"level":2,"title":"Unsupported Games","slug":"unsupported-games","link":"#unsupported-games","children":[{"level":3,"title":"Archipelago + Enemizer","slug":"archipelago-enemizer","link":"#archipelago-enemizer","children":[]},{"level":3,"title":"The ALTTP Randomizer","slug":"the-alttp-randomizer","link":"#the-alttp-randomizer","children":[]},{"level":3,"title":"Vanilla ALTTP Games","slug":"vanilla-alttp-games","link":"#vanilla-alttp-games","children":[]}]}],"git":{},"filePathRelative":"guide.md"}')}}]);