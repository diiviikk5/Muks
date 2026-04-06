import App from './App.svelte'
import { mount } from 'svelte'
import './styles/seelen/index.scss'

const app = mount(App, {
  target: document.getElementById('app'),
})

export default app
