// import 'ffi.dart' if (dart.library.html) 'ffi_web.dart';
import 'package:flutter/material.dart';
import 'package:flutter/cupertino.dart';
import 'package:app/src/rust/api/fa_api.dart';
// import 'package:provider/provider.dart';

class FARState extends ChangeNotifier {
  Data? data;
  bool updating = true;
  FaContext ctx = getContext();
  Fs_? fs;

  FARState() {
    refresh();
  }

  void home() {
    ctx.home();
    updateState();
  }

  void back() {
    ctx.back();
    updateState();
  }

  void transition(String text) {
    ctx.transition(text: text);
    updateState();
  }

  void refresh() {
    data = null;
    updating = true;
    ctx.home();
    fs = null;
    notifyListeners();

    getData().then((mlfs) {
      updating = false;
      data = mlfs;
      updateState();
    });
  }

  void updateState() {
    if (data == null) {
      return;
    }
    getState(data: data!, ctx: ctx).then((state) {
      fs = state;
      notifyListeners();
    });
  }
}
