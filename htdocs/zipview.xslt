<?xml version="1.0"?>
<xsl:stylesheet xmlns:xsl="http://www.w3.org/1999/XSL/Transform" version="1.0">

  <xsl:output method="html" />

  <xsl:template match="*" />

  <xsl:template match="zip">
    <html>
      <head>
        <title>
            <xsl:value-of select="@name" />
        </title>
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="stylesheet" type="text/css" href="/zipview.css" />
      </head>
      <body>
	      <h1>
          <xsl:value-of select="@name" />
	      </h1>
        <div class="wrapper">
			    <div class="dir">
			      <xsl:element name="a">
			        <xsl:attribute name="href">..</xsl:attribute>
			          <xsl:element name="img">
			            <xsl:attribute name="src">/icons/up.gif</xsl:attribute>
			          </xsl:element>
			          <xsl:text>..</xsl:text>
			      </xsl:element>
			    </div>
			    <xsl:apply-templates select="dir" />
			    <xsl:apply-templates select="file" />
        </div>
      </body>
    </html>
  </xsl:template>

  <xsl:template match="dir">
    <div class="dir">
      <xsl:element name="a">
        <xsl:attribute name="href">
	  			<xsl:value-of select="." />
        </xsl:attribute>
        <xsl:element name="img">
          <xsl:attribute name="src">/icons/dir.gif</xsl:attribute>
        </xsl:element>
        <xsl:value-of select="." />
      </xsl:element>
    </div>
    <!-- <xsl:apply-templates/ -->
  </xsl:template>

  <xsl:template match="file">
    <div class="file">
      <xsl:element name="a">
        <xsl:attribute name="href">
	        <xsl:value-of select="." />
        </xsl:attribute>
        <xsl:element name="img">
          <xsl:attribute name="src">/icons/generic.gif</xsl:attribute>
        </xsl:element>
        <xsl:value-of select="." />
      </xsl:element>
    </div>
    <!-- xsl:apply-templates/ -->
  </xsl:template>

</xsl:stylesheet>
