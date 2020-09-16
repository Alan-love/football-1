import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { SharedModule } from 'src/app/shared/shared.module';
import { LeagueDetailsComponent } from './details/league.details.component';
import { CommonModule } from '@angular/common';
import { RouterModule } from '@angular/router';
import { LeagueComponent } from './league.component';
import { ComponentsModule } from 'src/app/components/components.module';

@NgModule({
  declarations: [
    LeagueComponent,
    LeagueDetailsComponent
  ],
  imports: [
    CommonModule,
    SharedModule,
    RouterModule,
    BrowserModule,
    ComponentsModule
  ],
  providers: [
  ],
})
export class LeagueModule { } 
