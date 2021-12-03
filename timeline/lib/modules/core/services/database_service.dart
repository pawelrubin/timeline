import 'package:flutter/material.dart';
import 'package:hive/hive.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:timeline/modules/core/model/location_entry.dart';

class DatabaseService extends ChangeNotifier {
  List<LocationEntry> entries = [];
  Box<LocationEntry> box;

  DatabaseService({required this.box}) {
    box.listenable().addListener(() {
      entries = box.values.toList();
      notifyListeners();
    });
  }

  addLocation(LocationEntry location) {
    box.add(location);
  }
}
