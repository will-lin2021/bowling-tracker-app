// [Module] React
import * as React from 'react';

// [Module] NativeScript
import { Button, EventData, Observable } from '@nativescript/core';
import { ListView } from 'react-nativescript';

// [Module] React Navigation
import { RouteProp } from '@react-navigation/core';
import { FrameNavigationProp } from 'react-nativescript-navigation';

// [Module] Project
import { GameInfo } from '../models';
import { GameService } from '../services';
import { MainStackParamList } from './NavigationParamList';

type DateViewScreenProps = {
  route: RouteProp<MainStackParamList, 'DateView'>,
  navigation: FrameNavigationProp<MainStackParamList, 'DateView'>,
}

export function DateViewScreen({ route }: DateViewScreenProps) {
  const dateId = route.params.dateId;
  const games = GameService.getRecordById(dateId).games;

  function onTapDeleteButton(args: EventData) {
    const gameId = ((args.object as unknown as Button).bindingContext as GameInfo).game_id;

    GameService.deleteGameById(gameId);
  }

  function cellFactory(game: GameInfo) {
    return (
      <gridLayout
        height='auto'
        borderRadius='10'
        className='bg-secondary'
        rows='auto, auto'
        columns='*, auto'
        margin='5 10'
        padding='0'
      >
        <label
          row='0'
          column='0'
          fontWeight='700'
          className='text-primary'
          fontSize='18'
          text={`${game.game_no}`}
        />
        <button
          row='0'
          column='2'
          onTap={onTapDeleteButton}
          text="Delete"
        />

        <label
          row='1'
          columnSpan='2'
          fontWeight='700'
          className='text-primary'
          fontSize='18'
          text={`${game.score_str}`}
        />
      </gridLayout>
    )
  }

  return (
    <stackLayout
      height='100%'
    >
      <ListView
        items={games}
        cellFactory={cellFactory}
        separatorColor='transparent'
        height='100%'
      />
    </stackLayout>
  )
}
