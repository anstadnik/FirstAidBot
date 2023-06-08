import 'ffi.dart' if (dart.library.html) 'ffi_web.dart';
import 'package:flutter/material.dart';
import 'package:flutter/cupertino.dart';
// import 'package:provider/provider.dart';

class FARState extends ChangeNotifier {
  MultilangFs? faMLFS;
  RwLockFaContext faCTX = api.getContext();
  FAState? faState;

  FARState() {
    refresh();
  }

  void home() {
    api.home(ctx: faCTX);
    getState();
  }

  void back() {
    api.back(ctx: faCTX);
    getState();
  }

  void transition(String text) {
    api.transition(ctx: faCTX, text: text);
    getState();
  }

  void refresh() {
    faMLFS = null;
    faCTX = api.getContext();
    faState = null;
    notifyListeners();
    api.getData().then((mlfs) {
      faMLFS = mlfs;
      getState();
    });
  }

  void getState() {
    if (faMLFS != null) {
      // TODO: Change it
      api.getFs(mlfs: faMLFS!, ctx: faCTX).then((state) => faState = state);
    }
    notifyListeners();
  }
}
