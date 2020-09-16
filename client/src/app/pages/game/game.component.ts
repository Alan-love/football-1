import { Component, OnInit, OnDestroy } from '@angular/core';
import { FootballApi } from 'src/client/football.api.client';
import { ActivatedRoute } from '@angular/router';
import { HeaderService, HeaderModel } from './services/header.service';
import { Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';

@Component({
  templateUrl: './game.component.html'
})
export class GameComponent implements OnInit, OnDestroy {
  destroy$ = new Subject<void>();

  headerTitle: String = "";
  headerSubTitle: String = "";

  constructor(private api: FootballApi,
    private route: ActivatedRoute,
    public headerService: HeaderService) {
  }


  ngOnInit() {
    this.headerService.get()
      .pipe(takeUntil(this.destroy$))
      .subscribe((header: HeaderModel) => {
        setTimeout(() => {
          this.headerTitle = header.title;
          this.headerSubTitle = header.subTitle;
        });
      });
  }

  ngOnDestroy(): void {
    this.destroy$.next();
    this.destroy$.complete();
  }
}
