import 'package:flutter/material.dart';
import 'package:google_maps_flutter/google_maps_flutter.dart';
import 'package:timeline/modules/core/widgets/layout.dart';

class MapView extends StatefulWidget {
  const MapView({Key? key}) : super(key: key);

  @override
  State<StatefulWidget> createState() => _MapViewState();
}

class _MapViewState extends State<MapView> {
  final CameraPosition _initial = const CameraPosition(target: LatLng(0, 0));

  @override
  Widget build(BuildContext context) {
    return Layout(
      body: Center(
        child: GoogleMap(
          initialCameraPosition: _initial,
        ),
      ),
    );
  }
}
