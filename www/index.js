import * as ffxiv from 'ffxiv-squad-calc';

const currentLevel = document.getElementById('current-level');
const expToNext = document.getElementById('exp-to-next');

expToNext.innerText = ffxiv.calc_next_level_exp(currentLevel.value);

currentLevel.addEventListener('change', () => {
  expToNext.innerText = ffxiv.calc_next_level_exp(currentLevel.value);
});