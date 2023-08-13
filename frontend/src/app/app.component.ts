import { HttpClient } from '@angular/common/http';
import { Component } from '@angular/core';
import { BackendService } from './services/backend.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  title = 'frontend';

  constructor(private _backend: BackendService, private _http: HttpClient) { }

  public async onClick() {
    const response = this._backend.test(10, 20)

    response.subscribe(result => {
      console.log(result);
      console.log(result.res);
    })
  }
}
