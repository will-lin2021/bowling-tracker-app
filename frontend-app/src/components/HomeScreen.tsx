// [Module] React
import * as React from 'react';

// [Module] NativeScript
import { Button, ListView as NSCore_ListView, StackLayout, EventData, ItemEventData, SwipeGestureEventData, Observable } from '@nativescript/core';
import { ListView } from 'react-nativescript';

// [Module] React Navigation
import { RouteProp } from '@react-navigation/core';
import { FrameNavigationProp } from 'react-nativescript-navigation';

// [Module] Project
import { DateRecord } from '../models';
import { GameService } from '../services';
import { MainStackParamList } from './NavigationParamList';

type HomeScreenProps = {
  route: RouteProp<MainStackParamList, 'Home'>,
  navigation: FrameNavigationProp<MainStackParamList, 'Home'>,
}

export function HomeScreen({ navigation }: HomeScreenProps) {
  const dateStore = GameService.getDateStore();

  function cellFactory(record: DateRecord) {
    return (
      <gridLayout
        height='auto'
        borderRadius='10'
        className='bg-secondary'
        rows='auto, *, auto'
        columns='*, *, *'
        margin='5 10'
        padding='0'
      >
        <label
          row='0'
          column='0'
          fontWeight='700'
          className='text-primary'
          fontSize='18'
          text={`${record.date.getUTCFullYear()}/${record.date.getUTCMonth()+1}/${record.date.getUTCDate()}`}
        />
        <label
          row='0'
          column='2'
          fontWeight='700'
          className='text-primary'
          fontSize='18'
          text={`${record.games.length} game${record.games.length === 1 ? '' : 's'}`}
          textAlignment='right'
        />

        <label
          row='1'
          column='0'
          fontWeight='700'
          className='text-primary'
          fontSize='18'
          textWrap='true'
          textAlignment='center'
        >
          <formattedString>
            <span text={"Worst\n"}/>
            <span text={"60"}/>
          </formattedString>
        </label>
        <label
          row='1'
          column='1'
          fontWeight='700'
          className='text-primary'
          fontSize='18'
          textWrap='true'
          textAlignment='center'
        >
          <formattedString>
            <span text={'Avg\n'}/>
            <span text={'60'}/>
          </formattedString>
        </label>
        <label
          row='1'
          column='2'
          fontWeight='700'
          className='text-primary'
          fontSize='18'
          textWrap='true'
          textAlignment='center'
        >
          <formattedString>
            <span text={'Best\n'}/>
            <span text={'60'}/>
          </formattedString>
        </label>
      </gridLayout>
    )
  }

  function onButtonTapUpdate(args: EventData) {
    const stackLayout: StackLayout = (args.object as Button).parent as StackLayout;

    const listView: NSCore_ListView = stackLayout.getChildAt(2) as NSCore_ListView;

    listView.refresh();
  }

  function onButtonTapTest() {
    // GameService.testAddGame();
    GameService.test();
  }

  function onItemTap(args: ItemEventData) {
    const index = args.index;
    const date = dateStore.getItem(index);

    navigation.navigate('DateView', {
      dateId: (date as unknown as DateRecord).date_id,
    });
  }

  function onSwipe(args: SwipeGestureEventData) {
    console.log(args.direction);
  }

  return (
    <stackLayout
      height='100%'
    >
      <button
        text='Test'
        onTap={onButtonTapTest}
      />
      <button
        text='Update'
        onTap={onButtonTapUpdate}
      />
      <ListView
        items={dateStore}
        cellFactory={cellFactory}
        onItemTap={onItemTap}
        onSwipe={onSwipe}
        separatorColor='transparent'
        height='100%'
      />
    </stackLayout>
  )
}
