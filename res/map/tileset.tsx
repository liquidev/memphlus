<?xml version="1.0" encoding="UTF-8"?>
<tileset version="1.5" tiledversion="1.7.2" name="Reference Tiles" tilewidth="16" tileheight="16" tilecount="64" columns="8">
 <image source="../tileset.png" width="128" height="128"/>
 <tile id="0" type="empty"/>
 <tile id="1" type="solid_top_left">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="2" x="0" y="16">
    <point/>
   </object>
   <object id="3" x="0" y="0">
    <point/>
   </object>
   <object id="4" x="16" y="0">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="2" type="solid_top">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="3" x="0" y="0">
    <point/>
   </object>
   <object id="4" x="16" y="0">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="3" type="solid_top_right">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="0">
    <point/>
   </object>
   <object id="2" x="16" y="0">
    <point/>
   </object>
   <object id="3" x="16" y="16">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="4" type="solid_v_top">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="16">
    <point/>
   </object>
   <object id="2" x="0" y="0">
    <point/>
   </object>
   <object id="3" x="16" y="0">
    <point/>
   </object>
   <object id="4" x="16" y="16">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="5" type="slope_up">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="16">
    <point/>
   </object>
   <object id="2" x="16" y="0">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="6" type="slope_half_up_left">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="16">
    <point/>
   </object>
   <object id="2" x="16" y="8">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="7" type="slope_half_up_right">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="8">
    <point/>
   </object>
   <object id="2" x="16" y="0">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="8" type="spikes_up">
  <properties>
   <property name="deadly" type="bool" value="true"/>
  </properties>
 </tile>
 <tile id="9" type="solid_left">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="0">
    <point/>
   </object>
   <object id="2" x="0" y="16">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="10" type="empty">
  <properties>
   <property name="polyline" type="bool" value="false"/>
  </properties>
 </tile>
 <tile id="11" type="solid_right">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="16" y="16">
    <point/>
   </object>
   <object id="2" x="16" y="0">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="12" type="solid_v_middle">
  <properties>
   <property name="polyline" type="bool" value="false"/>
  </properties>
  <objectgroup draworder="index" id="4">
   <object id="5" x="0" y="0" width="16" height="16"/>
  </objectgroup>
 </tile>
 <tile id="13" type="slope_down">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="0">
    <point/>
   </object>
   <object id="2" x="16" y="16">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="14" type="slope_half_down_left">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="0">
    <point/>
   </object>
   <object id="2" x="16" y="8">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="15" type="slope_half_down_right">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="8">
    <point/>
   </object>
   <object id="2" x="16" y="16">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="16" type="spikes_right">
  <properties>
   <property name="deadly" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="0">
    <polygon points="0,0 8,4 0,8"/>
   </object>
   <object id="2" x="0" y="8">
    <polygon points="0,0 8,4 0,8"/>
   </object>
  </objectgroup>
 </tile>
 <tile id="17" type="solid_bottom_left">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="0">
    <point/>
   </object>
   <object id="2" x="0" y="16">
    <point/>
   </object>
   <object id="3" x="16" y="16">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="18" type="solid_bottom">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="16">
    <point/>
   </object>
   <object id="2" x="16" y="16">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="19" type="solid_bottom_right">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="16">
    <point/>
   </object>
   <object id="2" x="16" y="16">
    <point/>
   </object>
   <object id="3" x="16" y="0">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="20" type="solid_v_bottom">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="0">
    <point/>
   </object>
   <object id="2" x="0" y="16">
    <point/>
   </object>
   <object id="3" x="16" y="16">
    <point/>
   </object>
   <object id="4" x="16" y="0">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="21" type="slope_double_up_top">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="8" y="16">
    <point/>
   </object>
   <object id="2" x="16" y="0">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="22" type="slope_double_down_top">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="0">
    <point/>
   </object>
   <object id="2" x="8" y="16">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="24" type="spikes_down">
  <properties>
   <property name="deadly" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="0">
    <polygon points="0,0 4,8 8,0"/>
   </object>
   <object id="2" x="8" y="0">
    <polygon points="0,0 4,8 8,0"/>
   </object>
  </objectgroup>
 </tile>
 <tile id="25" type="solid_h_left">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="16" y="0">
    <point/>
   </object>
   <object id="2" x="0" y="0">
    <point/>
   </object>
   <object id="3" x="0" y="16">
    <point/>
   </object>
   <object id="4" x="16" y="16">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="26" type="solid_h_center">
  <properties>
   <property name="polyline" type="bool" value="false"/>
  </properties>
  <objectgroup draworder="index" id="3">
   <object id="2" x="0" y="0" width="16" height="16"/>
  </objectgroup>
 </tile>
 <tile id="27" type="solid_h_right">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="0">
    <point/>
   </object>
   <object id="2" x="16" y="0">
    <point/>
   </object>
   <object id="3" x="16" y="16">
    <point/>
   </object>
   <object id="4" x="0" y="16">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="28" type="solid_tile">
  <properties>
   <property name="polyline" type="bool" value="false"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="0" width="16" height="16"/>
  </objectgroup>
 </tile>
 <tile id="29" type="slope_double_up_bottom">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="16">
    <point/>
   </object>
   <object id="2" x="8" y="0">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="30" type="slope_double_down_bottom">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="8" y="0">
    <point/>
   </object>
   <object id="2" x="16" y="16">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="32" type="spikes_left">
  <properties>
   <property name="deadly" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="16" y="0">
    <polygon points="0,0 -8,4 0,8"/>
   </object>
   <object id="2" x="16" y="8">
    <polygon points="0,0 -8,4 0,8"/>
   </object>
  </objectgroup>
 </tile>
 <tile id="33" type="solid_top_fade_left">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="0">
    <point/>
   </object>
   <object id="2" x="16" y="0">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="34" type="solid_top_fade_right">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="0">
    <point/>
   </object>
   <object id="2" x="16" y="0">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="35" type="solid_bottom_fade_left">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="16">
    <point/>
   </object>
   <object id="2" x="16" y="16">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="36" type="solid_bottom_fade_right">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="16">
    <point/>
   </object>
   <object id="2" x="16" y="16">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="41" type="solid_left_fade_top">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="0">
    <point/>
   </object>
   <object id="2" x="0" y="16">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="42" type="solid_right_fade_top">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="16" y="0">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="49" type="solid_left_fade_bottom">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="0">
    <point/>
   </object>
   <object id="2" x="0" y="16">
    <point/>
   </object>
  </objectgroup>
 </tile>
 <tile id="50" type="solid_right_fade_bottom">
  <properties>
   <property name="polyline" type="bool" value="true"/>
  </properties>
 </tile>
 <tile id="63" type="barrier">
  <objectgroup draworder="index" id="3">
   <object id="2" x="0" y="0" width="16" height="16"/>
  </objectgroup>
 </tile>
</tileset>
