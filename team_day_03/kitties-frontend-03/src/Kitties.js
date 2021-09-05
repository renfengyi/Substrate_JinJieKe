import React, { useEffect, useLayoutEffect, useState } from 'react'
import { Form, Grid } from 'semantic-ui-react'

import { useSubstrate } from './substrate-lib'
import { TxButton } from './substrate-lib/components'

import KittyCards from './KittyCards'

export default function Kitties (props) {
  const { api, keyring } = useSubstrate()
  const { accountPair } = props

  const [kitties, setKitties] = useState([])
  const [status, setStatus] = useState('')

  const [count,setCount] = useState()
  const [dnaarr,setDnaarr] = useState([])
  const [ownerarr,setOwnerarr] = useState([])
  const [pop, setPop] = useState(false)



  const fetchKitties = async () => {
    // TODO: 在这里调用 `api.query.kittiesModule.*` 函数去取得猫咪的信息。
    // 你需要取得：
    //   - 共有多少只猫咪
    //   - 每只猫咪的主人是谁
    //   - 每只猫咪的 DNA 是什么，用来组合出它的形态

    let kittiesList = [];
    let ownerList = [];  

    api.query.kittiesModule.kittiesCount(d=>{
      if(d.toHuman()>0){
        console.log(d.toHuman());
        setCount(d.toHuman());
        for(let i=0;i<d.toHuman();i++){
          api.queryMulti([
            [api.query.kittiesModule.owner,i],
            [api.query.kittiesModule.kitties,i]
          ],([ov,dv])=>{
            kittiesList.push(dv.unwrap().toU8a());
            //console.log("kittiesArr--"+JSON.stringify(kittiesArr))
            setDnaarr(kittiesList);
            ownerList.push(ov.unwrap().toString());
            //console.log("ownerArr--"+JSON.stringify(ownerArr))
            setOwnerarr(ownerList);

            

          });
        }
      }
    });


  }

  const populateKitties = () => {
    // TODO: 在这里添加额外的逻辑。你需要组成这样的数组结构：
    //  ```javascript
    //  const kitties = [{
    //    id: 0,
    //    dna: ...,
    //    owner: ...
    //  }, { id: ..., dna: ..., owner: ... }]
    //  ```
    // 这个 kitties 会传入 <KittyCards/> 然后对每只猫咪进行处理
    let _kitties = [];

    //console.log("POP--",JSON.stringify(dnaarr),JSON.stringify(ownerarr));
    setTimeout(()=>{
      console.log("owner---"+JSON.stringify(ownerarr));
      for(let i=0;i<count;i++){
        let arr = {
          id:i,
          dna:dnaarr[i],
          owner:ownerarr[i]
        };
        _kitties.push(arr);
        //console.log(JSON.stringify(_kitties))
      }
      setTimeout(()=>{
        setKitties(_kitties);
      },300);
    },300);

  }

  useEffect(fetchKitties, [api, keyring,status])
  useLayoutEffect(populateKitties, [dnaarr,ownerarr])

  return <Grid.Column width={16}>
    <h1>小毛孩</h1>
    <KittyCards kitties={kitties} accountPair={accountPair} setStatus={setStatus}/>
    <Form style={{ margin: '1em 0' }}>
      <Form.Field style={{ textAlign: 'center' }}>
        <TxButton
          accountPair={accountPair} label='创建小毛孩' type='SIGNED-TX' setStatus={setStatus}
          attrs={{
            palletRpc: 'kittiesModule',
            callable: 'create',
            inputParams: [],
            paramFields: []
          }}
        />
      </Form.Field>
    </Form>
    <div style={{ overflowWrap: 'break-word' }}>{status}</div>
  </Grid.Column>
}
