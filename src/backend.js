import { invoke } from '@tauri-apps/api/tauri'

export default {
  async hello (a, bC) {
    return invoke('hello', {
      a,
      bC: bC
    })
  },

  async initTable () {
    return invoke('init_table')
  },

  async addImagesMeta (meta) {
    return invoke('add_images_meta', meta)
  },
  
  async getImagesMetaList (page, pageSize) {
    const data = {
      page: page,
      pageSize: pageSize
    }
    return invoke('get_images_meta_list', data)
  }
}