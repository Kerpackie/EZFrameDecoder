<template>
  <n-card embedded>
    <n-h3 class="m-0">New Command Builder</n-h3>

    <!-- ───── 1. metadata ───── -->
    <n-form :model="cmd" label-width="120" class="mb-5">
      <n-form-item label="Letter">
        <n-input v-model:value="cmd.letter" maxlength="1" placeholder="A-Z" />
      </n-form-item>
      <n-form-item label="Description">
        <n-input v-model:value="cmd.description" />
      </n-form-item>
    </n-form>

    <!-- ───── 2. items list ───── -->
    <div v-for="(item, idx) in cmd.items" :key="idx" class="mb-3 item-block">

      <!-- ── GROUP ── -->
      <n-card
          v-if="item.kind === 'group'"
          size="small"
          :title="item.name || 'Group'"
          :closable="idx !== 0"
          @close="removeItem(idx)"
      >
        <n-form label-width="110" class="mb-3">
          <n-form-item label="Group name">
            <n-input v-model:value="item.name" :disabled="idx === 0"/>
          </n-form-item>
        </n-form>

        <FieldEditor v-model:fields="item.fields" />

      </n-card>

      <!-- ── SWITCH ── -->
      <n-card
          v-else
          size="small"
          :title="`${item.switch_key} switch`"
          closable
          @close="removeItem(idx)"
      >
        <n-form label-width="110" class="mb-3">
          <n-form-item label="Switch key">
            <n-input v-model:value="item.switch_key" />
          </n-form-item>
        </n-form>

        <!-- cases -->
        <div v-for="(cs, key) in item.cases" :key="key" class="mb-2 case-block">
          <n-card size="small" :title="`Case ${key}`" closable
                  @close="deleteCase(idx, key)">
            <n-input v-model:value="cs.description"
                     placeholder="Case description (optional)"
                     class="mb-2"/>
            <FieldEditor v-model:fields="cs.groups" />
          </n-card>
        </div>

        <n-space>
          <n-button tertiary size="tiny" @click="addCase(idx)">+ Add case</n-button>
          <n-button tertiary size="tiny" @click="setDefault(idx)">Set default</n-button>
        </n-space>
      </n-card>
    </div>

    <!-- add buttons -->
    <n-space class="mb-4">
      <n-button @click="addGroup">+ Add Group</n-button>
      <n-button @click="addSwitch">+ Add Switch</n-button>
    </n-space>

    <!-- ───── 3. validation / preview ───── -->
    <n-alert v-if="errors.length" type="error" :show-icon="false" class="mb-3">
      <div v-for="e in errors" :key="e">{{ e }}</div>
    </n-alert>

    <n-text depth="3" class="mb-2">
      Remaining hex chars: {{ remaining }}
    </n-text>

    <n-code :code="preview" language="json" class="mb-4"/>

    <n-button type="primary" @click="submit"
              :disabled="submitDisabled">Submit</n-button>
    <n-button class="ml-2" @click="reset">Reset</n-button>

    <!-- reference -->
    <n-divider>Valid data sizes</n-divider>
    <n-table :bordered="false" size="small" style="max-width:520px">
      <thead><tr><th>Len</th><th>Bytes</th><th>Mapped type</th></tr></thead>
      <tbody>
      <tr><td>2</td><td>1</td><td>u8 / bool</td></tr>
      <tr><td>4</td><td>2</td><td>u16</td></tr>
      <tr><td>6</td><td>3</td><td>u24</td></tr>
      <tr><td>8</td><td>4</td><td>u32</td></tr>
      <tr><td>16</td><td>8</td><td>u64</td></tr>
      </tbody>
    </n-table>

  </n-card>
</template>

<script setup lang="ts">
import {
  NCard, NForm, NFormItem, NInput, NButton,
  NSpace, NAlert, NText, NDivider, NCode, NTable, useMessage
} from "naive-ui";
import { ref, computed } from "vue";
import FieldEditor from "./FieldEditor.vue";
import { invoke } from "@tauri-apps/api/core";

/* ---------- helpers ---------- */
const LEN_MAP = { 2:"u8",4:"u16",6:"u24",8:"u32",16:"u64" } as Record<number,string>;
const MAX = 21, MIN = 18;
const msg = useMessage();

/* ---------- types ---------- */
interface Field { name:string; len:number; base?:10|16; type:"number"|"bool"; description?:string; }
interface Group  { kind:"group"; name:string; fields:Field[]; }
interface SwitchCase { description?:string; groups:FieldGroup[]; }
interface SwitchItem { kind:"switch"; switch_key:string; cases:Record<string,SwitchCase>; }
type FieldGroup = Group;
type Item = Group | SwitchItem;

/* ---------- state ---------- */
const cmd = ref<{letter:string;description:string;items:Item[]}>({
  letter:"",
  description:"",
  items:[
    { kind:"group", name:"Header", fields:[{name:"RSAddress",len:2,base:16,type:"number",description:"Device addr"}] }
  ]
});

/* ---------- add/remove ---------- */
function addGroup(){
  cmd.value.items.push({kind:"group",name:"Group",fields:[]});
}
function addSwitch(){
  cmd.value.items.push({kind:"switch",switch_key:"Opcode",cases:{}});
}
function removeItem(i:number){ if(i!==0) cmd.value.items.splice(i,1); }

function addCase(swIdx:number){
  const key = prompt("Hex case (e.g. 0x5002)")?.trim();
  if(!key || !/^0x[0-9a-fA-F]+$/.test(key)){ msg.warning("Invalid"); return;}
  const sw=cmd.value.items[swIdx] as SwitchItem;
  if(sw.cases[key]){msg.warning("Exists");return;}
  sw.cases[key]={groups:[]};
}
function deleteCase(swIdx:number,key:string){
  delete (cmd.value.items[swIdx] as SwitchItem).cases[key];
}
function setDefault(swIdx:number){
  const defDesc = prompt("Default description (optional)")||"";
  const sw=cmd.value.items[swIdx] as SwitchItem;
  (sw as any).default={description:defDesc,groups:[]};
}

/* ---------- flatten fields ---------- */
function allFields():Field[]{
  const list:Field[]=[];
  cmd.value.items.forEach(it=>{
    if(it.kind==="group") list.push(...it.fields);
    else Object.values(it.cases).forEach(c=>c.groups.forEach(g=>list.push(...g.fields)));
  });
  return list;
}

/* ---------- length + errors ---------- */
const total = computed(()=> allFields().reduce((s,f)=>s+f.len,0));
const remaining = computed(()=> MAX-total.value);
const errors = computed(()=>{
  const e:string[]=[];
  for(const f of allFields()){
    if(f.type==="bool" && f.len!==2) e.push(`Bool "${f.name}" len must be 2`);
    if(f.type==="number" && !(f.len in LEN_MAP)) e.push(`Field "${f.name}" len ${f.len} not 2/4/6/8/16`);
  }
  if(total.value<MIN||total.value>MAX) e.push(`Frame length ${total.value} must be ${MIN}-${MAX}`);
  return e;
});
const submitDisabled = computed<boolean>(() =>
    errors.value.length > 0 ||
    total.value < MIN ||
    total.value > MAX
);

/* ---------- mapped JSON ---------- */
const preview = computed(()=> JSON.stringify({
  letter:cmd.value.letter.toUpperCase(),
  description:cmd.value.description,
  items: cmd.value.items.map(it=>{
    if(it.kind==="group"){
      return {
        name:it.name,
        fields: it.fields.map(f=>({
          name:f.name,
          len:f.len,
          base:f.type==="bool"?undefined:f.base,
          type: f.type==="bool"?"bool":LEN_MAP[f.len],
          description:f.description
        }))
      };
    }
    // switch
    const sw=it as SwitchItem;
    const cs:Record<string,any>={};
    for(const [k,v] of Object.entries(sw.cases)){
      cs[k]={description:v.description,groups:v.groups.map(g=>({
          name:g.name,fields:g.fields.map(f=>({
            name:f.name,len:f.len,base:f.type==="bool"?undefined:f.base,
            type:f.type==="bool"?"bool":LEN_MAP[f.len],description:f.description
          }))
        }))};
    }
    const obj:any={switch:sw.switch_key,cases:cs};
    if((sw as any).default) obj.default=(sw as any).default;
    return obj;
  })
},null,2));

/* ---------- submit ---------- */
async function submit(){
  if(errors.value.length){msg.error("Fix errors first");return;}
  if(!/^[A-Z]$/i.test(cmd.value.letter)){msg.error("Letter A-Z");return;}
  try{
    await invoke("append_command",{ newCmd: JSON.parse(preview.value) });
    msg.success("Saved!");
  }catch(e:any){ msg.error(String(e));}
}

/* ---------- reset ---------- */
function reset(){
  cmd.value.letter="";cmd.value.description="";
  cmd.value.items=[{
    kind:"group",name:"Header",
    fields:[{name:"RSAddress",len:2,base:16,type:"number",description:"Device addr"}]
  }];
}
</script>

<style scoped>
.item-block{border-left:4px solid #eee;padding-left:1rem}
.case-block{margin-left:1rem}
</style>
